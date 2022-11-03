use crate::db::DB_POOL;
use crate::models::entity::*;
use crate::utils::crypt;
use log::error;
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::ops::DerefMut;
use rbatis::crud;

crud!(Article{});
crud!(SiteInfo{});
crud!(Request{});
crud!(Tags{});
crud!(Category{});
crud!(Reward{});
crud!(Comment{});
crud!(User{});
crud!(Like{});
crud!(Link{});
crud!(Resource{});


#[cfg(test)]
mod test {
    use std::ops::DerefMut;
    use log::kv::Source;
    use rbatis::executor::RbatisRef;
    use crate::models::entity::*;
    use crate::db::DB_POOL;
    use rbatis::rbdc::datetime::FastDateTime;

    #[actix_rt::test]
    async fn test_auth() {
        let cate = Category{
            id: None,
            created_at: Some(FastDateTime::now()),
            updated_at: Some(FastDateTime::now()),
            is_delete: Some(false),
            name: Some("ssss".to_string()),
            display_name: Some("ssss".to_string()),
            seo_desc: Some("ssss".to_string()),
            support: Some(false),
            parent_id: Some(0)
        };
        let mut rb = DB_POOL.acquire().await.unwrap();
        Category::insert(&mut rb, &cate).await.expect("TODO: panic message");
    }
}
