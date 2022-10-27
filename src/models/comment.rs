use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<i64>,
    pub create_at: Option<FastDateTime>,
    pub update_at: Option<FastDateTime>,
    pub is_delete: Option<bool>,
    pub user_id: Option<i32>,
    pub article_id: Option<i32>,
    pub content: Option<String>,
    pub parent_id: Option<i32>,
    pub ip: Option<String>,
    pub ua: Option<String>,
    pub location: Option<String>,
    pub os: Option<String>,
}
rbatis::crud!(Comment {},"comment"); // this way custom table name
