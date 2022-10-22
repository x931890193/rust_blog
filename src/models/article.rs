
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<u32>,
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
//crud = async fn insert(...)+async fn  select_by_column(...)+ async fn  update_by_column(...)+async fn  delete_by_column(...)...and more
rbatis::crud!(Article {},"article"); // this way custom table name
