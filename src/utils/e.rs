use lazy_static::lazy_static;
use std::any::TypeId;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{write, Display, Formatter};

type GenError = Box<dyn Error>;
pub type GenResult<T> = Result<T, GenError>;

lazy_static! {
    pub static ref AUTH_ERROR: i32 = 1;
    pub static ref PARAMS_ERROR: i32 = 2;
    pub static ref DB_ERROR: i32 = 3;
    pub static ref LOGIC_ERROR: i32 = 4;
    pub static ref UNKNOWN: i32 = 5;
    pub static ref ERROR_MSG: HashMap<i32, String> = {
        let mut err_msg = HashMap::new();
        err_msg.insert(1, String::from("认证失败"));
        err_msg.insert(2, String::from("参数错误"));
        err_msg.insert(3, String::from("数据库异常"));
        err_msg.insert(4, String::from("逻辑错误"));
        err_msg.insert(5, String::from("未知错误"));
        err_msg
    };
}

#[derive(Debug, Clone)]
pub struct AuthError {
    base_error: BaseError,
    code: i32
}

impl AuthError {
    fn new(message: &str) -> AuthError {
        AuthError{
            code: 1,
            base_error: BaseError::new(message),
            }
        }

    fn as_base(&self) -> &BaseError {
        &self.base_error
    }

    fn as_mut_base(&mut self) -> &mut BaseError {
        &mut self.base_error
    }
}


#[derive(Debug, Clone)]
pub struct BaseError {
    message: String,
}

impl BaseError {
    pub fn new(message: &str) -> BaseError {
        BaseError{
            message: message.to_string()
        }
    }
}

impl Display for BaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BaseError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod test {
    use crate::utils::e::{AuthError, GenResult};

    #[test]
    fn test_error(){
        // let res = GenResult::Err(Box::new(AuthError::new(""))).unwrap();
        // println!("{}", res.err())
    }
}
