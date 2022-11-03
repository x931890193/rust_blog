use rbatis::rbdc::datetime::FastDateTime;

use std::ops::DerefMut;

use serde::{Deserialize, Serialize};

use crate::db::DB_POOL;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub category_id: Option<u32>,
    pub tag: Option<String>,
    pub user_id: Option<u32>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub click_times: Option<u32>,
    pub like_count: Option<i32>,
    pub collect_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub weight: Option<u32>,
    pub support: Option<bool>,
    pub header_img_type: Option<u32>,
    pub header_img: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct SiteInfo {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub key_words: Option<String>,
    pub description: Option<String>,
    pub record_number: Option<String>,
    pub ali_pay_image: Option<String>,
    pub wechat_pay_image: Option<String>,
    pub self_description: Option<String>,
    pub self_description_html: Option<String>,
    pub git: Option<String>,
    pub job: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub ip: Option<String>,
    pub referer: Option<String>,
    pub url: Option<String>,
    pub major: Option<String>,
    pub remote_addr: Option<String>,
    pub user_agent: Option<String>,
    pub op_type: Option<String>,
    pub method: Option<String>,
    pub is_login: Option<String>,
    pub request_time: Option<u64>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Tags {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub name: Option<String>,
    pub tag_type: Option<u32>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub seo_desc: Option<String>,
    pub support: Option<bool>,
    pub parent_id: Option<u32>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Reward {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub order_id: Option<String>,
    pub who: Option<String>,
    pub amount: Option<f64>,
    pub payment_method: Option<u32>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub user_id: Option<i64>,
    pub article_id: Option<i64>,
    pub content: Option<String>,
    pub parent_id: Option<i64>,
    pub ip: Option<String>,
    pub ua: Option<String>,
    pub location: Option<String>,
    pub os: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub label: Option<String>,
    pub email: Option<String>,
    pub github_id: Option<i64>,
    pub github_url: Option<String>,
    pub is_admin: Option<bool>,
    pub receive_update: Option<bool>,
    pub last_login: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Like {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub user_id: Option<i64>,
    pub article_id: Option<i64>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub header_img: Option<String>,
    pub show_link: Option<bool>,
    pub verify_status: Option<i32>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<i64>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub uuid: Option<String>,
    pub key: Option<String>,
    pub r#type: Option<i32>,
}
