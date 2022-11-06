use lazy_static::lazy_static;
use rsa::Hash;
use std::collections::HashMap;
use std::io;

use crate::utils::{e, request};

lazy_static! {
    pub static ref gitHubAccessTokenUrl: String = "https://github.com/login/oauth/access_token";
    pub static ref githubUserInfoUrl: String = "https://api.github.com/user";
}

struct AccessTokenGetRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}
struct AccessTokenResp {
    access_token: String,
    scope: String,
    token_type: String,
    redirect_uri: String,
}

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

pub fn get_access_token(code: &str) -> Result<AccessTokenResp, io::Error> {
    unimplemented!();
    // TODO
    let mut params: HashMap<&str, &str> =
        HashMap::from([("client_id", "2"), ("client_secret", "4")]);
    let mut url_parameters = gitHubAccessTokenUrl.to_string() + "?";
    for (k, v) in params {
        url_parameters += &format!("{}={}", k.to_string(), v)
    }
    let res = request::post(
        url_parameters,
        None,
        Some(HashMap::from([("Accept".to_string(), "application/json".to_string())])),
    );
}

pub fn get_user_info(token: &str) -> Result<UserInfo, io::Error> {
    unimplemented!();
    let res = request::get(
        githubUserInfoUrl.to_string(),
        None,
        Some(HashMap::from(vec![("Authorization".to_string(), format!("Bearer {}", token).to_string())])),
    );
    if let Some(user) = serde_json::from_value::<UserInfo>(res) {
        Ok(user)
    }else {
        Box::new(e::BaseError::new("get res"))
    }
}
