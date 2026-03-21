//! Tests for object operations

mod common;

use huaweicloud_sdk_rust_obs::ObsError;
use std::env;

/// 综合集成测试：测试所有对象操作
/// 按顺序测试：put_object -> get_object -> head_object -> copy_object -> append_object -> delete_object
#[tokio::test]
async fn test_object_operations_integration() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    
    // 使用唯一前缀避免测试冲突（使用微秒级时间戳）
    let test_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
    
    println!("========== 开始对象操作集成测试 ==========");
    println!("测试 ID: {}", test_id);

    // ========================================
    // 1. 测试 put_object
    // ========================================
    println!("\n[1/6] 测试 put_object...");
    let put_key = format!("test-put-{}.txt", test_id);
    let put_content = b"Hello, OBS! This is a test object.";
    
    let put_result = obs.put_object()
        .bucket(&bucket)
        .key(&put_key)
        .body(put_content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;
    
    println!("  ✓ put_object 成功");
    println!("    Key: {}", put_key);
    println!("    ETag: {:?}", put_result.etag());
    assert!(put_result.etag().is_some(), "ETag 应该存在");

    // ========================================
    // 2. 测试 get_object
    // ========================================
    println!("\n[2/6] 测试 get_object...");
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(&put_key)
        .send()
        .await?;
    
    let body = get_result.body();
    assert_eq!(body.as_ref(), put_content, "获取的内容应与上传的内容一致");
    println!("  ✓ get_object 成功");
    println!("    Content-Length: {:?}", get_result.content_length());
    println!("    Content-Type: {:?}", get_result.content_type());
    println!("    ETag: {:?}", get_result.etag());

    // ========================================
    // 3. 测试 head_object
    // ========================================
    println!("\n[3/6] 测试 head_object...");
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(&put_key)
        .send()
        .await?;
    
    println!("  ✓ head_object 成功");
    println!("    Content-Length: {:?}", head_result.content_length());
    println!("    Content-Type: {:?}", head_result.content_type());
    println!("    ETag: {:?}", head_result.etag());
    println!("    Last-Modified: {:?}", head_result.last_modified());
    
    // 验证 head 返回的元数据与 get 一致
    assert_eq!(get_result.content_length(), head_result.content_length());
    assert_eq!(get_result.content_type(), head_result.content_type());

    // ========================================
    // 4. 测试 copy_object
    // ========================================
    println!("\n[4/6] 测试 copy_object...");
    let copy_key = format!("test-copy-{}.txt", test_id);
    
    let copy_result = obs.copy_object()
        .bucket(&bucket)
        .key(&copy_key)
        .copy_source(format!("{}/{}", bucket, put_key))
        .content_type("text/plain")
        .send()
        .await?;
    
    println!("  ✓ copy_object 成功");
    println!("    Source: {}", put_key);
    println!("    Destination: {}", copy_key);
    println!("    ETag: {}", copy_result.etag());
    println!("    Last-Modified: {}", copy_result.last_modified());
    
    // 验证复制后的对象内容一致
    let copied_get_result = obs.get_object()
        .bucket(&bucket)
        .key(&copy_key)
        .send()
        .await?;
    
    assert_eq!(copied_get_result.body().as_ref(), put_content, "复制的内容应与原内容一致");

    // ========================================
    // 5. 测试 append_object
    // ========================================
    println!("\n[5/6] 测试 append_object...");
    let append_key = format!("test-append-{}.txt", test_id);
    
    // 第一次追加（position=0 创建新文件）
    let append1 = obs.append_object()
        .bucket(&bucket)
        .key(&append_key)
        .position(0)
        .body(b"First part. ".to_vec())
        .send()
        .await?;
    
    println!("  ✓ append_object 第一次追加成功");
    println!("    Next position: {:?}", append1.next_position());
    
    // 等待一秒确保时间戳不同
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 第二次追加
    let next_pos = append1.next_position().expect("应该返回 next_position");
    let append2 = obs.append_object()
        .bucket(&bucket)
        .key(&append_key)
        .position(next_pos)
        .body(b"Second part.".to_vec())
        .send()
        .await?;
    
    println!("  ✓ append_object 第二次追加成功");
    println!("    Next position: {:?}", append2.next_position());
    
    // 验证追加后的内容
    let append_get_result = obs.get_object()
        .bucket(&bucket)
        .key(&append_key)
        .send()
        .await?;
    
    let expected_content = b"First part. Second part.";
    assert_eq!(
        append_get_result.body().as_ref(),
        expected_content,
        "追加后的内容应为 'First part. Second part.'"
    );

    // ========================================
    // 6. 测试 delete_object
    // ========================================
    println!("\n[6/6] 测试 delete_object...");
    
    // 删除所有创建的对象
    let keys_to_delete = vec![
        put_key.clone(),
        copy_key.clone(),
        append_key.clone(),
    ];
    
    for key in &keys_to_delete {
        obs.delete_object()
            .bucket(&bucket)
            .key(key)
            .send()
            .await?;
        println!("  ✓ delete_object 成功: {}", key);
    }
    
    // 验证对象已被删除
    println!("\n验证对象已被删除...");
    for key in &keys_to_delete {
        let result = obs.head_object()
            .bucket(&bucket)
            .key(key)
            .send()
            .await;
        
        assert!(result.is_err(), "对象 {} 应该已被删除", key);
        println!("  ✓ 对象已确认删除: {}", key);
    }

    println!("\n========== 所有对象操作测试通过 ==========");
    Ok(())
}

/// 单独测试 put_object 和 get_object
#[tokio::test]
async fn test_put_and_get_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-object.txt";
    let content = b"Hello, OBS!";

    // Put object
    let put_result = obs.put_object()
        .bucket(&bucket)
        .key(key)
        .body(content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("Put object: {}", key);
    if let Some(etag) = put_result.etag() {
        println!("ETag: {}", etag);
    }

    // Get object
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Get object: {}", key);
    println!("Content length: {:?}", get_result.content_length());
    println!("Content type: {:?}", get_result.content_type());

    let body = get_result.body();
    assert_eq!(body.as_ref(), content);

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Deleted object: {}", key);

    Ok(())
}

/// 单独测试 head_object
#[tokio::test]
async fn test_head_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-head-object.txt";
    let content = b"Test content for head operation";

    // Put object
    obs.put_object()
        .bucket(&bucket)
        .key(key)
        .body(content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    // Head object
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Head object: {}", key);
    println!("Content length: {:?}", head_result.content_length());
    println!("Content type: {:?}", head_result.content_type());
    println!("ETag: {:?}", head_result.etag());

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

/// 单独测试 copy_object
#[tokio::test]
async fn test_copy_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let src_key = "test-copy-source.txt";
    let dest_key = "test-copy-dest.txt";
    let content = b"Content to be copied";

    // Put source object
    obs.put_object()
        .bucket(&bucket)
        .key(src_key)
        .body(content.to_vec())
        .send()
        .await?;

    // Copy object
    let copy_result = obs.copy_object()
        .bucket(&bucket)
        .key(dest_key)
        .copy_source(format!("{}/{}", bucket, src_key))
        .send()
        .await?;

    println!("Copied object from {} to {}", src_key, dest_key);
    println!("ETag: {}", copy_result.etag());

    // Verify the copy
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(dest_key)
        .send()
        .await?;

    assert_eq!(get_result.body().as_ref(), content);

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(src_key)
        .send()
        .await?;

    obs.delete_object()
        .bucket(&bucket)
        .key(dest_key)
        .send()
        .await?;

    Ok(())
}

/// 单独测试 append_object
#[tokio::test]
async fn test_append_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    // 使用唯一的 key 避免与之前的测试冲突（使用毫秒级时间戳）
    let key = format!("test-append-{}.txt", chrono::Utc::now().timestamp_millis());

    // First append
    let append1 = obs.append_object()
        .bucket(&bucket)
        .key(&key)
        .position(0)
        .body(b"Hello, ".to_vec())
        .send()
        .await?;

    println!("First append, next position: {:?}", append1.next_position());

    // 等待一秒确保时间戳不同
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Second append
    let next_pos = append1.next_position().unwrap_or(7);
    let append2 = obs.append_object()
        .bucket(&bucket)
        .key(&key)
        .position(next_pos)
        .body(b"World!".to_vec())
        .send()
        .await?;

    println!("Second append, next position: {:?}", append2.next_position());

    // Verify the content
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await?;

    let body = get_result.body();
    assert_eq!(body.as_ref(), b"Hello, World!");

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await?;

    Ok(())
}

/// 单独测试 delete_object
#[tokio::test]
async fn test_delete_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-delete-object.txt";

    // 先创建对象
    obs.put_object()
        .bucket(&bucket)
        .key(key)
        .body(b"test content for delete".to_vec())
        .send()
        .await?;
    
    println!("Created object: {}", key);

    // 验证对象存在
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;
    assert!(head_result.is_ok(), "对象应该存在");

    // 删除对象
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;
    
    println!("Deleted object: {}", key);

    // 验证对象已被删除
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await;
    assert!(head_result.is_err(), "对象应该已被删除");

    Ok(())
}

/// 测试批量删除对象
#[tokio::test]
async fn test_delete_objects() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");

    // Create multiple objects
    let keys = vec![
        "test-delete-1.txt".to_string(),
        "test-delete-2.txt".to_string(),
        "test-delete-3.txt".to_string(),
    ];

    for key in &keys {
        obs.put_object()
            .bucket(&bucket)
            .key(key)
            .body(b"test content".to_vec())
            .send()
            .await?;
    }

    // Delete multiple objects (non-quiet mode)
    let result = obs.delete_objects()
        .bucket(&bucket)
        .keys(keys.clone())
        .quiet(false)
        .send()
        .await?;

    println!("Deleted {} objects", result.deleted().len());
    println!("Errors: {} objects", result.errors().len());

    // Verify all objects were deleted successfully
    assert!(result.is_all_success(), "All objects should be deleted successfully");
    assert_eq!(result.deleted().len(), 3, "Should have 3 deleted objects");

    // Verify each deleted object
    for deleted in result.deleted() {
        println!("  Deleted: {}", deleted.key());
        assert!(keys.contains(&deleted.key().to_string()), "Deleted key should be in the original keys");
    }

    // Verify objects are deleted
    for key in &keys {
        let result = obs.head_object()
            .bucket(&bucket)
            .key(key)
            .send()
            .await;

        // Should fail because object doesn't exist
        assert!(result.is_err());
    }

    Ok(())
}

/// 测试批量删除对象 - quiet 模式
#[tokio::test]
async fn test_delete_objects_quiet_mode() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");

    // Create multiple objects
    let keys = vec![
        "test-delete-quiet-1.txt".to_string(),
        "test-delete-quiet-2.txt".to_string(),
    ];

    for key in &keys {
        obs.put_object()
            .bucket(&bucket)
            .key(key)
            .body(b"test content".to_vec())
            .send()
            .await?;
    }

    // Delete multiple objects (quiet mode)
    let result = obs.delete_objects()
        .bucket(&bucket)
        .keys(keys.clone())
        .quiet(true)  // Only return errors
        .send()
        .await?;

    println!("Quiet mode - Errors: {} objects", result.errors().len());

    // In quiet mode, deleted list should be empty if all succeeded
    // Only errors are returned
    assert!(result.is_all_success(), "All objects should be deleted successfully");

    // Verify objects are deleted
    for key in &keys {
        let result = obs.head_object()
            .bucket(&bucket)
            .key(key)
            .send()
            .await;
        assert!(result.is_err());
    }

    Ok(())
}

/// 测试批量删除对象 - 包含不存在的对象
#[tokio::test]
async fn test_delete_objects_with_nonexistent() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");

    // Create one object
    let existing_key = "test-delete-existing.txt";
    obs.put_object()
        .bucket(&bucket)
        .key(existing_key)
        .body(b"test content".to_vec())
        .send()
        .await?;

    // Try to delete existing and non-existing objects
    let keys = vec![
        existing_key.to_string(),
        "nonexistent-object-1.txt".to_string(),
        "nonexistent-object-2.txt".to_string(),
    ];

    let result = obs.delete_objects()
        .bucket(&bucket)
        .keys(keys.clone())
        .quiet(false)
        .send()
        .await?;

    println!("Deleted {} objects", result.deleted().len());
    println!("Errors: {} objects", result.errors().len());

    // OBS returns success for non-existent objects too (they just don't exist anymore)
    // So all should be in deleted list
    assert_eq!(result.deleted().len(), 3, "All objects should be in deleted list");

    // Clean up - verify the existing object is deleted
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(existing_key)
        .send()
        .await;
    assert!(head_result.is_err(), "Existing object should be deleted");

    Ok(())
}

/// 测试对象 ACL 操作
#[tokio::test]
async fn test_object_acl() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = format!("test-acl-{}.txt", chrono::Utc::now().timestamp_millis());

    // 1. 创建一个对象
    obs.put_object()
        .bucket(&bucket)
        .key(&key)
        .body(b"test content for acl".to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("Created object: {}", key);

    // 2. 获取对象 ACL（默认 ACL）
    let acl_result = obs.get_object_acl()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await?;

    println!("Get object ACL:");
    println!("  Owner ID: {}", acl_result.owner().id());
    println!("  Delivered: {}", acl_result.delivered());
    println!("  Grants count: {}", acl_result.grants().len());

    // 验证默认 ACL 有至少一个 grant（owner 有 FULL_CONTROL）
    assert!(!acl_result.grants().is_empty(), "Default ACL should have at least one grant");

    for grant in acl_result.grants() {
        println!("  Grant:");
        if let Some(id) = grant.grantee.id() {
            println!("    Grantee ID: {}", id);
        }
        if let Some(canned) = grant.grantee.canned() {
            println!("    Grantee Canned: {:?}", canned);
        }
        println!("    Permission: {:?}", grant.permission);
    }

    // 3. 设置对象 ACL（使用预定义 ACL）
    // 注意：实际测试可能需要根据您的 OBS 配置调整
    let set_result = obs.set_object_acl()
        .bucket(&bucket)
        .key(&key)
        .canned_acl("private")
        .send()
        .await?;

    println!("Set object ACL to private");
    if let Some(version_id) = set_result.version_id() {
        println!("  Version ID: {}", version_id);
    }

    // 4. 再次获取 ACL 验证设置成功
    let acl_result2 = obs.get_object_acl()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await?;

    println!("Get object ACL after set:");
    println!("  Owner ID: {}", acl_result2.owner().id());
    println!("  Grants count: {}", acl_result2.grants().len());

    // 5. 清理
    obs.delete_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await?;

    println!("Deleted object: {}", key);

    Ok(())
}
