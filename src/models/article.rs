use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = article)]
pub struct Article {
    pub id: i64,
    // pub create_at: Option<FastDateTime>,
    // pub update_at: Option<FastDateTime>,
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

