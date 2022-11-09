use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use serde;

use crate::utils::{e, request};

lazy_static! {
    pub static ref GIT_HUB_ACCESS_TOKEN_URL: String = "https://github.com/login/oauth/access_token".to_string();
    pub static ref GITHUB_USER_INFO_URL: String = "https://api.github.com/user".to_string();
}

struct AccessTokenGetRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccessTokenResp {
    access_token: String,
    scope: String,
    token_type: String,
    redirect_uri: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserInfo {
    login: String,
    id: i128,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    event_url: String,
    receive_event_url: String,
    r#type: String,
    site_admin: String,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: String,
    hireable: String,
    bio: String,
    twitter_user_name: String,
    public_repos: i128,
    public_gists: i128,
    followers: i128,
    following: i128,
    created_at: String,
    updated_at: String,
}

pub async fn get_access_token(code: &str) -> Result<AccessTokenResp, io::Error> {
    let mut params: HashMap<&str, &str> =
        HashMap::from([("client_id", "2"), ("client_secret", "4"), ("code", code), ("redirect_uri", "")]);
    let mut url_parameters = GIT_HUB_ACCESS_TOKEN_URL.to_string() + "?";
    for (k, v) in params {
        url_parameters += &format!("{}={}", k.to_string(), v)
    }
    let res = request::post(
        url_parameters,
        None,
        Some(HashMap::from([("Accept".to_string(), "application/json".to_string())])),
    ).await;
    match res {
        Ok(token) => {
            let token: AccessTokenResp = token.json().await.unwrap();
            println!("{:?}", token);
            Ok(token)
        },
        Err(err) => {
            Err(io::Error::new(io::ErrorKind::ConnectionRefused, "error"))
        }
    }
}

pub async fn get_user_info(token: &str) -> Result<UserInfo, io::Error> {
    let res = request::get(
        GITHUB_USER_INFO_URL.to_string(),
        None,
        Some(HashMap::from([("Authorization".to_string(), format!("Bearer {}", token).to_string())])),
    ).await;
    match res {
        Ok(user) => {
            let user: UserInfo = user.json().await.unwrap();
            println!("{:?}", user);
            Ok(user)
        },
        Err(err) => {
            Err(io::Error::new(io::ErrorKind::ConnectionRefused, "error"))
        }
    }
}


#[cfg(test)]
mod test {
    #[actix_rt::test] // 异步测试 need
    #[test]
    async fn test_get() {
        println!("11111")
    }
}