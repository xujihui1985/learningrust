use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, DATE};

use super::oss::OSS;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

static CONTENT_MD5_STR: &'static str = "Content-MD5";

pub trait Auth {
    fn oss_sign(
        &self,
        verb: &str,
        bucket: &str,
        object: &str,
        oss_resources: &str,
        headers: &HeaderMap,
    ) -> String;
}

impl Auth for OSS {
    fn oss_sign(
        &self,
        verb: &str,
        bucket: &str,
        object: &str,
        sub_resources: &str,
        headers: &HeaderMap<HeaderValue>,
    ) -> String {
        let date = headers
            .get(DATE)
            .map(|d| d.to_str().unwrap_or_default())
            .unwrap_or_default();
        let content_type = headers
            .get(CONTENT_TYPE)
            .map(|d| d.to_str().unwrap_or_default())
            .unwrap_or_default();
        let content_md5 = headers
            .get(CONTENT_MD5_STR)
            .map(|md5| md5.to_str().unwrap_or_default())
            .unwrap_or_default();

        let mut oss_headers: Vec<(&HeaderName, &HeaderValue)> = headers
            .iter()
            .filter(|(k, _)| k.as_str().contains("x-oss-"))
            .collect();

        oss_headers.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));
        let mut oss_headers_str = String::new();
        for (k, v) in oss_headers {
            oss_headers_str += &format!(
                "{}:{}\n",
                k.to_owned().as_str(),
                v.to_owned().to_str().unwrap_or("")
            );
        }
        let oss_resource_str = get_oss_resource_str(bucket, object, sub_resources);
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            verb, content_md5, content_type, date, oss_headers_str, oss_resource_str
        );
        let mut hasher = Hmac::new(Sha1::new(), self.get_access_key_secret().as_bytes());
        hasher.input(sign_str.as_bytes());
        let sign_str_base64 = base64::encode(hasher.result().code());
        let authorization = format!("OSS {}:{}", self.get_access_key_id(), sign_str_base64);
        authorization
    }
}

fn get_oss_resource_str(bucket: &str, object: &str, sub_resources: &str) -> String {
    let oss_resources = if sub_resources != "" {
        String::from("?") + sub_resources
    } else {
        String::new()
    };
    if bucket == "" {
        format!("/{}{}", bucket, oss_resources)
    } else {
        format!("/{}/{}{}", bucket, object, oss_resources)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::get_oss_resource_str;

    #[test]
    fn test_get_oss_resource_str() {
        let res = get_oss_resource_str("test_bucket", "test_object", "oss_resources");
        assert_eq!("/test_bucket/test_object?oss_resources", res);
    }
}
