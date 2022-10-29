use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = article)]
pub struct Article {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub category_id: u32,
    pub tag: String,
    pub user_id: u32,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub click_times: u32,
    pub like_count: i32,
    pub collect_count: i32,
    pub comment_count: i32,
    pub weight: u32,
    pub support: bool,
    pub header_img_type: u32,
    pub header_img: String,
}

impl Article {}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = siteinfo)]
pub struct SiteInfo {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub author: String,
    pub title: String,
    pub key_words: String,
    pub description: String,
    pub record_number: String,
    pub ali_pay_image: String,
    pub wechat_pay_image: String,
    pub self_description: String,
    pub self_description_html: String,
    pub git: String,
    pub job: String,
}


#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = request)]
pub struct Request {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub ip: String,
    pub referer: String,
    pub url: String,
    pub major: String,
    pub remote_addr: String,
    pub user_agent: String,
    pub op_type: String,
    pub method: String,
    pub is_login: String,
    pub request_time: u64,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = tags)]
pub struct Tags {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub name: String,
    pub tag_type: u32,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub name: String,
    pub display_name: String,
    pub seo_desc: String,
    pub support: bool,
    pub parent_id: u32,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = reward)]
pub struct Reward {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub order_id: String,
    pub who: String,
    pub amount: f64,
    pub payment_method: u32,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = comment)]
pub struct Comment {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub user_id: i64,
    pub article_id: i64,
    pub content: String,
    pub parent_id: i64,
    pub ip: String,
    pub ua: String,
    pub location: String,
    pub os: String,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub user_name: String,
    pub password: String,
    pub avatar: String,
    pub label: String,
    pub email: String,
    pub github_id: i64,
    pub github_url: String,
    pub is_admin: bool,
    pub receive_update: bool,
    pub last_login: String,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = like)]
pub struct Like {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub user_id: i64,
    pub article_id: i64,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = link)]
pub struct Link {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub email: String,
    pub url: String,
    pub header_img: String,
    pub show_link: bool,
    pub verify_status: i32,
}


#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = resource)]
pub struct Resource {
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub is_delete: bool,
    pub uuid: String,
    pub key: String,
    pub r#type:  i32,
}