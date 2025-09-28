use crate::{client::Client, config::SignatureType, error::ObsError};
use ::base64::{engine::general_purpose, Engine};
use chrono::{DateTime, TimeZone, Utc};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::{collections::HashMap, str::FromStr};
use hmac_sha1::hmac_sha1;
const RFC1123: &str = "%a, %d %b %Y %H:%M:%S GMT";


pub trait Authorization {
    fn signature(
        &self,
        method: &str,
        params: HashMap<String, String>,
        headers: HashMap<String, Vec<String>>,
        canonicalized_url: String,
    ) -> Result<String, ObsError>;

    fn auth(
        &self,
        method: &str,
        bucket: &str,
        params: HashMap<String, String>,
        headers: HashMap<String, Vec<String>>,
        canonicalized_url: String,
    ) -> Result<HeaderMap, ObsError>;
}

impl Authorization for Client {

    #[allow(clippy::useless_vec)]
    fn signature(
        &self,
        method: &str,
        _params: HashMap<String, String>,
        headers: HashMap<String, Vec<String>>,
        canonicalized_url: String,
    ) -> Result<String, ObsError> {
        let attach_headers = attach_headers(headers, true);

        let string_to_sign = vec![
            method, //HTTP-Verb
            "\n",
            &attach_headers, // Content-MD5 \n Content-Type \n
            "\n",
            &canonicalized_url,
        ]
        .join("");

        // println!("string_to_sign: {:?}", &string_to_sign);

        let security = self.security();
        match security {
            Some(s) => {
                let signature = signature(&string_to_sign, s.sk())?;
                Ok(signature)
            }
            None => Err(ObsError::Security),
        }
    }

    fn auth(
        &self,
        method: &str,
        bucket: &str,
        params: HashMap<String, String>,
        mut headers: HashMap<String, Vec<String>>,
        canonicalized_url: String,
    ) -> Result<HeaderMap, ObsError> {
        let is_v4 = matches!(self.config().signature_type, SignatureType::V4);
        if !bucket.is_empty() {
            headers.insert(
                "Host".into(),
                vec![format!("{}.{}", bucket, self.config().endpoint())],
            );
        } else {
            headers.insert("Host".into(), vec![self.config().endpoint().to_string()]);
        }

        prepare_host_and_date(&mut headers, self.config().endpoint(), is_v4);

        let sign = self.signature(method, params, headers.clone(), canonicalized_url)?;

        let security = self.security();
        match security {
            Some(s) => {
                let value = format!("OBS {}:{}", s.ak(), sign);
                let mut h = HeaderMap::new();
                h.insert(
                    "Authorization",
                    HeaderValue::from_str(value.as_str()).unwrap(),
                );
                for (key, value) in headers.iter() {
                    h.insert(
                        HeaderName::from_str(key).unwrap(),
                        HeaderValue::from_str(&value.join(",")).unwrap(),
                    );
                }
                Ok(h)
            }
            None => Err(ObsError::Security),
        }
    }
}

fn prepare_host_and_date(
    headers: &mut HashMap<String, Vec<String>>,
    _host_name: &str,
    is_v4: bool,
) {
    if let Some(date) = headers.get("x-amz-date") {
        let mut flag = false;
        if date.len() == 1 {
            if is_v4 {
                // 20060102T150405Z
                if let Ok(t) = DateTime::parse_from_str(&date[0], "%Y%m%dT%H%M%SZ") {
                    headers.insert("Date".into(), vec![t.format(RFC1123).to_string()]);
                    flag = true;
                }
            } else if date[0].ends_with("GMT") {
                headers.insert("Date".into(), vec![date[0].clone()]);
                flag = true;
            }
        }
        if !flag {
            headers.remove("x-amz-date");
        }
    }
    if !headers.contains_key("Date") {
        headers.insert("Date".into(), vec![Utc::now().format(RFC1123).to_string()]);
    }
}

// fn encode_headers(headers: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
//     headers
//         .into_iter()
//         .map(|(key, values)| {
//             (
//                 key,
//                 values
//                     .iter()
//                     .map(|v| urlencoding::encode(v).to_string())
//                     .collect::<Vec<String>>(),
//             )
//         })
//         .collect::<HashMap<String, Vec<String>>>()
// }

// fn signature_header(
//     headers: HashMap<String, Vec<String>>,
// ) -> (Vec<String>, HashMap<String, Vec<String>>) {
//     let mut signed_headers = vec![];
//     let mut headers2 = HashMap::with_capacity(headers.len());
//     for (key, value) in headers {
//         let key2 = key.trim().to_lowercase();
//         if !key2.is_empty() {
//             signed_headers.push(key2.clone());
//             headers2.insert(key2, value);
//         }
//     }
//     signed_headers.sort();
//     (signed_headers, headers2)
// }

// fn credential(ak: &str, region: &str, short_date: &str) -> (String, String) {
//     let scope = format!("{}/{}/{}/{}", short_date, region, "s3", "aws4_request");
//     // return fmt.Sprintf("%s/%s", ak, scope), scope
//     (format!("{}/{}", ak, &scope), scope)
// }

fn string_to_sign(
    keys: Vec<String>,
    is_obs: bool,
    headers: HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut sign = Vec::with_capacity(keys.len());

    for key in keys {
        let prefix_header = if is_obs { "x-obs-" } else { "x-amz-" };
        let prefix_meta_header = if is_obs { "x-obs-meta-" } else { "x-amz-meta-" };
        let mut value = String::new();
        if key.starts_with(prefix_header) {
            if key.starts_with(prefix_meta_header) {
                let header_value = headers.get(&key).unwrap();
                for (index, val) in header_value.iter().enumerate() {
                    value.push_str(val.trim());
                    if index != header_value.len() - 1 {
                        value.push(',');
                    }
                }
            } else {
                value = headers.get(&key).unwrap().join(",")
            }
            value = format!("{}:{}", key, &value);
        } else {
            value = headers.get(&key).unwrap().join(",");
        }
        sign.push(value);
    }
    sign
}

fn attach_headers(headers: HashMap<String, Vec<String>>, is_obs: bool) -> String {
    let mut _headers = HashMap::with_capacity(headers.len());
    let mut keys = vec![];
    let headers: HashMap<String, Vec<String>> = headers
        .into_iter()
        .filter(|(key, _)| !key.trim().to_lowercase().is_empty())
        .collect::<_>();
    for (key, value) in headers {
        let _key = key.trim().to_lowercase();
        let prefix_header = if is_obs { "x-obs-" } else { "x-amz-" };
        if _key == "content-md5"
            || _key == "content-type"
            || _key == "date"
            || _key.starts_with(prefix_header)
        {
            keys.push(_key.clone());
            _headers.insert(_key, value);
        }
    }

    for interested_header in ["content-md5", "content-type", "date"] {
        if !_headers.contains_key(interested_header) {
            _headers.insert(interested_header.into(), vec![]);
            keys.push(interested_header.into());
        }
    }

    let date_camel_header = if is_obs { "X-obs-Date" } else { "X-Amz-Date" };
    let data_header = date_camel_header.to_lowercase();

    if _headers.contains_key(&data_header) || _headers.contains_key(date_camel_header) {
        _headers.insert(date_camel_header.into(), vec![rfc_1123()]);
    }

    keys.sort();

    let to_sign = string_to_sign(keys, is_obs, _headers);
    to_sign.join("\n")
}

/// 签名，算法如下:
/// > Signature = Base64( HMAC-SHA1( YourSecretAccessKeyID, UTF-8-Encoding-Of( StringToSign ) ) )
fn signature(string_to_sign: &str, sk: &str) -> Result<String, ObsError> {
    let hash = hmac_sha1(sk.as_bytes(), string_to_sign.as_bytes());
    let hs = general_purpose::STANDARD.encode(hash);
    Ok(hs)
}

fn rfc_1123() -> String {
    let date = Utc::now().format(RFC1123).to_string();
    date
}
