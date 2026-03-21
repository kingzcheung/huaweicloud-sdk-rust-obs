//! Tests for multipart upload operations

mod common;

use huaweicloud_sdk_rust_obs::{CompletedPart, ObsError};
use std::env;

/// 综合集成测试：测试分段上传操作
/// 按顺序测试：initiate_multipart_upload -> upload_part -> list_parts -> complete_multipart_upload
#[tokio::test]
async fn test_multipart_upload_integration() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    
    // 使用唯一前缀避免测试冲突（使用纳秒级时间戳）
    let test_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    
    println!("========== 开始分段上传集成测试 ==========");
    println!("测试 ID: {}", test_id);

    let object_key = format!("test-multipart-{}.bin", test_id);

    // ========================================
    // 1. 测试 initiate_multipart_upload
    // ========================================
    println!("\n[1/5] 测试 initiate_multipart_upload...");
    let initiate_result = obs.initiate_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .content_type("application/octet-stream")
        .send()
        .await?;
    
    let upload_id = initiate_result.upload_id().to_string();
    println!("  ✓ initiate_multipart_upload 成功");
    println!("    Bucket: {}", initiate_result.bucket());
    println!("    Key: {}", initiate_result.key());
    println!("    UploadId: {}", upload_id);
    
    assert!(!upload_id.is_empty(), "UploadId 不应为空");

    // ========================================
    // 2. 测试 upload_part (上传多个段)
    // ========================================
    println!("\n[2/5] 测试 upload_part...");
    
    // 创建测试数据（每段 100KB）
    let part1_data: Vec<u8> = (0..100 * 1024).map(|i| (i % 256) as u8).collect();
    let part2_data: Vec<u8> = (0..100 * 1024).map(|i| ((i + 1) % 256) as u8).collect();
    let part3_data: Vec<u8> = (0..50 * 1024).map(|i| ((i + 2) % 256) as u8).collect();
    
    // 上传第1段
    let upload_part1_result = obs.upload_part()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .part_number(1)
        .body(part1_data.clone())
        .send()
        .await?;
    
    let etag1 = upload_part1_result.etag().to_string();
    println!("  ✓ upload_part 1 成功");
    println!("    PartNumber: {}", upload_part1_result.part_number());
    println!("    ETag: {}", etag1);
    
    // 上传第2段
    let upload_part2_result = obs.upload_part()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .part_number(2)
        .body(part2_data.clone())
        .send()
        .await?;
    
    let etag2 = upload_part2_result.etag().to_string();
    println!("  ✓ upload_part 2 成功");
    println!("    PartNumber: {}", upload_part2_result.part_number());
    println!("    ETag: {}", etag2);
    
    // 上传第3段
    let upload_part3_result = obs.upload_part()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .part_number(3)
        .body(part3_data.clone())
        .send()
        .await?;
    
    let etag3 = upload_part3_result.etag().to_string();
    println!("  ✓ upload_part 3 成功");
    println!("    PartNumber: {}", upload_part3_result.part_number());
    println!("    ETag: {}", etag3);

    // ========================================
    // 3. 测试 list_parts
    // ========================================
    println!("\n[3/5] 测试 list_parts...");
    let list_parts_result = obs.list_parts()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .send()
        .await?;
    
    println!("  ✓ list_parts 成功");
    println!("    Bucket: {}", list_parts_result.bucket());
    println!("    Key: {}", list_parts_result.key());
    println!("    UploadId: {}", list_parts_result.upload_id());
    println!("    Parts count: {}", list_parts_result.parts().len());
    
    assert_eq!(list_parts_result.parts().len(), 3, "应该有3个段");
    
    // 验证段信息
    for (i, part) in list_parts_result.parts().iter().enumerate() {
        println!("      Part {}: number={}, size={}, etag={}", 
            i + 1, part.part_number(), part.size(), part.etag());
    }

    // ========================================
    // 4. 测试 complete_multipart_upload
    // ========================================
    println!("\n[4/5] 测试 complete_multipart_upload...");
    let complete_result = obs.complete_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .part(1, &etag1)
        .part(2, &etag2)
        .part(3, &etag3)
        .send()
        .await?;
    
    println!("  ✓ complete_multipart_upload 成功");
    println!("    Location: {}", complete_result.location());
    println!("    Bucket: {}", complete_result.bucket());
    println!("    Key: {}", complete_result.key());
    println!("    ETag: {}", complete_result.etag());

    // ========================================
    // 5. 验证合并后的对象
    // ========================================
    println!("\n[5/5] 验证合并后的对象...");
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;
    
    let body = get_result.body();
    let expected_size = part1_data.len() + part2_data.len() + part3_data.len();
    assert_eq!(body.len(), expected_size, "对象大小应该等于所有段大小之和");
    
    // 验证内容
    let body_ref = body.as_ref();
    assert_eq!(&body_ref[0..part1_data.len()], part1_data.as_slice(), "第1段内容应该匹配");
    assert_eq!(&body_ref[part1_data.len()..part1_data.len() + part2_data.len()], part2_data.as_slice(), "第2段内容应该匹配");
    assert_eq!(&body_ref[part1_data.len() + part2_data.len()..], part3_data.as_slice(), "第3段内容应该匹配");
    
    println!("  ✓ 对象验证成功");
    println!("    Total size: {} bytes", body.len());

    // 清理测试对象
    println!("\n清理测试对象...");
    obs.delete_object()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;
    println!("  ✓ 清理完成");

    println!("\n========== 分段上传集成测试完成 ==========");
    Ok(())
}

/// 测试取消分段上传
#[tokio::test]
async fn test_abort_multipart_upload() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    
    let test_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    
    println!("========== 开始取消分段上传测试 ==========");
    println!("测试 ID: {}", test_id);

    let object_key = format!("test-abort-multipart-{}.bin", test_id);

    // 1. 初始化分段上传
    println!("\n[1/4] 初始化分段上传...");
    let initiate_result = obs.initiate_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;
    
    let upload_id = initiate_result.upload_id().to_string();
    println!("  ✓ 初始化成功，UploadId: {}", upload_id);

    // 2. 上传一个段
    println!("\n[2/4] 上传一个段...");
    let part_data: Vec<u8> = (0..10 * 1024).map(|i| (i % 256) as u8).collect();
    
    obs.upload_part()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .part_number(1)
        .body(part_data)
        .send()
        .await?;
    
    println!("  ✓ 段上传成功");

    // 3. 取消分段上传
    println!("\n[3/4] 取消分段上传...");
    obs.abort_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .send()
        .await?;
    
    println!("  ✓ 取消成功");

    // 4. 验证分段上传已被取消（list_parts 应该失败）
    println!("\n[4/4] 验证分段上传已取消...");
    let list_result = obs.list_parts()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .send()
        .await;
    
    // 取消后 list_parts 应该返回错误
    assert!(list_result.is_err(), "取消后 list_parts 应该失败");
    println!("  ✓ 验证成功：分段上传已被取消");

    println!("\n========== 取消分段上传测试完成 ==========");
    Ok(())
}

/// 测试列举分段上传任务
#[tokio::test]
async fn test_list_multipart_uploads() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    
    let test_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    
    println!("========== 开始列举分段上传任务测试 ==========");
    println!("测试 ID: {}", test_id);

    let object_key = format!("test-list-uploads-{}.bin", test_id);

    // 1. 初始化一个分段上传任务
    println!("\n[1/3] 初始化分段上传任务...");
    let initiate_result = obs.initiate_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;
    
    let upload_id = initiate_result.upload_id().to_string();
    println!("  ✓ 初始化成功，UploadId: {}", upload_id);

    // 2. 列举分段上传任务
    println!("\n[2/3] 列举分段上传任务...");
    let list_result = obs.list_multipart_uploads()
        .bucket(&bucket)
        .prefix(&format!("test-list-uploads-{}", test_id))
        .max_uploads(10)
        .send()
        .await?;
    
    println!("  ✓ 列举成功");
    println!("    Bucket: {}", list_result.bucket());
    println!("    MaxUploads: {}", list_result.max_uploads());
    println!("    IsTruncated: {}", list_result.is_truncated());
    println!("    Uploads count: {}", list_result.uploads().len());
    
    // 验证我们创建的任务在列表中
    let found = list_result.uploads().iter().any(|u| u.upload_id() == upload_id);
    assert!(found, "创建的分段上传任务应该在列表中");
    
    for upload in list_result.uploads() {
        println!("      - Key: {}, UploadId: {}", upload.key(), upload.upload_id());
    }

    // 3. 清理：取消分段上传
    println!("\n[3/3] 清理分段上传任务...");
    obs.abort_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .send()
        .await?;
    
    println!("  ✓ 清理完成");

    println!("\n========== 列举分段上传任务测试完成 ==========");
    Ok(())
}

/// 测试使用 CompletedPart 结构体
#[tokio::test]
async fn test_multipart_upload_with_completed_parts() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    
    let test_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    
    println!("========== 开始测试 CompletedPart 结构体 ==========");
    println!("测试 ID: {}", test_id);

    let object_key = format!("test-completed-parts-{}.bin", test_id);

    // 初始化分段上传
    let initiate_result = obs.initiate_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;
    
    let upload_id = initiate_result.upload_id().to_string();

    // 上传段并收集 CompletedPart
    let mut completed_parts: Vec<CompletedPart> = Vec::new();
    
    for part_num in 1..=3 {
        let part_data: Vec<u8> = (0..10 * 1024).map(|i| ((i + part_num) % 256) as u8).collect();
        
        let result = obs.upload_part()
            .bucket(&bucket)
            .key(&object_key)
            .upload_id(&upload_id)
            .part_number(part_num)
            .body(part_data)
            .send()
            .await?;
        
        completed_parts.push(CompletedPart::new(part_num, result.etag()));
        println!("  ✓ 上传段 {} 成功", part_num);
    }

    // 使用 parts 方法设置所有段
    let complete_result = obs.complete_multipart_upload()
        .bucket(&bucket)
        .key(&object_key)
        .upload_id(&upload_id)
        .parts(completed_parts)
        .send()
        .await?;
    
    println!("  ✓ 合并成功，ETag: {}", complete_result.etag());

    // 清理
    obs.delete_object()
        .bucket(&bucket)
        .key(&object_key)
        .send()
        .await?;

    println!("========== CompletedPart 结构体测试完成 ==========");
    Ok(())
}