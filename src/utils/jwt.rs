use chrono::Local;
use jsonwebtoken;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static;
use redis::ErrorKind;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::models::entity::User;

lazy_static::lazy_static!(
    pub static ref JWT_SECRET: String = "secret".to_string();
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_info: User,
    exp: usize,
}

pub fn generate_jwt(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let expire_time = Local::now().timestamp() + 86400 * 7;
    let claims = Claims {
        user_info: user,
        exp: expire_time as usize,
    };
    let token = Box::new(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?);
    Ok(*token)
}

// just decode not decrypt
pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .unwrap();
    let res = &claims.claims;
    if res.exp < Local::now().timestamp() as usize {
        let err =
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature);
        return Err(err);
    }
    Ok(claims.claims)
}

#[cfg(test)]
mod test {
    use super::generate_jwt;
    use crate::models::entity::User;
    use crate::utils::jwt::verify_jwt;
    use actix_web::web::to;

    #[test]
    fn test_jwt() {
        let u = User {
            id: None,
            created_at: None,
            updated_at: None,
            is_delete: None,
            user_name: None,
            password: None,
            avatar: None,
            label: None,
            email: None,
            github_id: None,
            github_url: None,
            is_admin: None,
            receive_update: None,
            last_login: None,
        };
        let token = generate_jwt(u).unwrap();
        println!("{:}", token);
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2luZm8iOnsiaWQiOjEsImNyZWF0ZWRfYXQiOiIyMDIyLTA4LTE1IDE5OjU5OjU5LjU0MyIsInVwZGF0ZWRfYXQiOiIyMDIyLTEwLTEzIDIxOjI2OjMwLjkwNiIsImlzX2RlbGV0ZSI6ZmFsc2UsInVzZXJfbmFtZSI6InNhaW50IiwicGFzc3dvcmQiOiI2MzFlNGMyNzEyNzM4OWVlMGYzOGJmMDBiMjU5ZDZhMjYxZjFhNGNlNzhiMWJmYmMwNmViMDY4OTU1YjVjZWE2IiwiYXZhdGFyIjoiaHR0cHM6Ly9hdmF0YXJzLmdpdGh1YnVzZXJjb250ZW50LmNvbS91LzI0NjMyNTc0P3Y9NCIsImxhYmVsIjo1LCJlbWFpbCI6InlzdWRxZnNAMTYzLmNvbSIsImdpdGh1Yl9pZCI6MjQ2MzI1NzQsImdpdGh1Yl91cmwiOiJodHRwczovL2FwaS5naXRodWIuY29tL3VzZXJzL3g5MzE4OTAxOTMiLCJpc19hZG1pbiI6dHJ1ZSwicmVjZWl2ZV91cGRhdGUiOnRydWUsImxhc3RfbG9naW4iOiIyMDIyLTEwLTEzIDIxOjI2OjMwLjg5NiJ9LCJleHAiOjE2NjkwMTc3NTN9.jy0mfp5GpK4nyl-ybraxH1uitjg5UAji-RVZjA-HHmU";
        let c = verify_jwt(&token);
        println!("{:?}", c.unwrap().user_info);
    }
}
