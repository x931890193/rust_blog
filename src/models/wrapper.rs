use crate::db::DB_POOL;
use crate::utils::e;
use crate::models::entity::*;
use crate::utils::crypt;
use crate::utils::jwt;
use rbatis;
use std::error::Error;
use std::fmt::Debug;

rbatis::crud!(Article {});
rbatis::crud!(SiteInfo {});
rbatis::crud!(Request {});
rbatis::crud!(Tags {});
rbatis::crud!(Category {});
rbatis::crud!(Reward {});
rbatis::crud!(Comment {});
rbatis::crud!(User {});
rbatis::crud!(Like {});
rbatis::crud!(Link {});
rbatis::crud!(Resource {});

// nice design
rbatis::impl_select!(Category{select_by_name(display_name:&str, name:&str) => "`where display_name = #{display_name} and name = #{name} limit 1;`"});

rbatis::impl_select!(User{verify_user(table_name:&str, user_name:&str, password:&str) -> Option => "`where user_name = #{user_name} and password = #{password} limit 1;`"});

//
pub struct WrapperUser {
    pub use_name: String,
    pub password: String,
}

impl WrapperUser {
    pub fn new(user_name: &str, password: &str) -> WrapperUser {
        WrapperUser {
            use_name: user_name.to_string(),
            password: crypt::sha256(password),
        }
    }

    pub async fn auth_user(&self) -> Result<String, rbatis::Error> {
        match self.authenticate().await {
            Ok(token) => Ok(token),
            Err(error) => Err(error),
        }
    }

    async fn authenticate(&self) -> Result<String, rbatis::Error> {
        let mut rb = DB_POOL.acquire().await?;
        return match User::verify_user(&mut rb, r#""user""#, &self.use_name, &self.password).await?
        {
            Some(user) => {
                let token = jwt::generate_jwt(user);
                match token {
                    Ok(token) => Ok(token),
                    Err(err) => Err(rbatis::Error::from("generate jwt token error!")),
                }
            }
            None => Err(rbatis::Error::from("user not exist!")),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::db::DB_POOL;
    use crate::models::entity::*;
    use log::kv::Source;
    use rbatis::executor::RbatisRef;
    use rbatis::rbdc::datetime::FastDateTime;
    use std::ops::DerefMut;

    #[actix_rt::test]
    async fn test_auth() {
        let cate = Category {
            id: None,
            created_at: Some(FastDateTime::now()),
            updated_at: Some(FastDateTime::now()),
            is_delete: Some(false),
            name: Some("ssss".to_string()),
            display_name: Some("ssss".to_string()),
            seo_desc: Some("ssss".to_string()),
            support: Some(false),
            parent_id: Some(0),
        };
        let mut rb = DB_POOL.acquire().await.unwrap();
        // Category::insert(&mut rb, &cate)
        //     .await
        //     .expect("TODO: panic message");
        // let data = Category::select_by_name(&mut rb, "ssss",  "111111").await;
        // println!("{:?}", data.unwrap());
        let user = User::verify_user(
            &mut rb,
            r#""user""#,
            "saint",
            "4c281d23e295052237e4f62e0b987a1c9251a9f57df824841e3ecba96cff7863",
        )
        .await;
        println!("{:?}", user)
    }
}
