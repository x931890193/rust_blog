use std::time;
use std::str::FromStr;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::error::Error;

use reqwest;
use reqwest::header;
use log;



lazy_static!(
    pub static ref USER_AGENT: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36".to_string();
    pub static ref JSON_TYPE: String = "application/json".to_string();
    pub static ref PROTOBUF_TYPE: String = "application/protobuf".to_string();
    pub static ref TIME_OUT: time::Duration = time::Duration::new(10, 0);
);

pub enum ContentType {
    Json(String),
    Protobuf(String)
}

fn get_client(extra_headers: HashMap<String, String>) -> reqwest::Client {
    let mut headers_map = header::HeaderMap::new();
    for (k, v) in extra_headers {
        headers_map.insert(header::HeaderName::from_str(k.as_str()).unwrap(), header::HeaderValue::from_str(v.as_str()).unwrap());
    }
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT.as_str())
        .timeout(TIME_OUT.clone())
        .default_headers(headers_map)
        .build()
        .unwrap();
    client
}

pub async fn get(url: String, params: HashMap<String, String>, extra_headers: HashMap<String, String>) -> Result<reqwest::Response, Box<dyn Error>> {
    let mut url_with_parameters = url.as_str().to_owned() + "?";
    for (k, v) in params {
        url_with_parameters += &format!("{}={}", k, v)
    }
    let client = get_client(extra_headers);
    let res = client
        .get(url.clone())
        .send()
        .await?;
    let res = res.error_for_status().unwrap();
    log::info!("request: {}, status_code: {}", url.clone().as_str(), res.status());
    Ok(res)
}

// only json
// data serde_json::json!({k: v})
pub async fn post(url: String, data: serde_json::Value, extra_headers: HashMap<String, String>) -> Result<reqwest::Response, Box<dyn Error>> {
    let client = get_client(extra_headers);
    let res = client
        .post(url.clone())
        .json(&data)
        .send()
        .await?;
    let res = res.error_for_status().unwrap();
    log::error!("request: {}, status_code: {}", url.clone().as_str(), res.status());
    Ok(res)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::utils::request::{ContentType, get, post};
    use crate::utils::request::JSON_TYPE;

    #[actix_rt::test] // 异步测试 need
    #[test]
    async fn test_get() {
        let mut p = HashMap::new();
        p.insert("wd".to_string(),"s".to_string());
        let res = get("https://www.baidu.com/s".to_string(), p.clone(), HashMap::new());
        println!("{:?}", res.await)
    }

    #[actix_rt::test] // 异步测试 need
    #[test]
    async fn test_post() {
        let mut p = HashMap::new();
        p.insert("wd".to_string(),"s".to_string());
        let res = post("https://www.baidu.com/s".to_string(), serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }), HashMap::new());
        println!("{:?}", res.await)
    }
}