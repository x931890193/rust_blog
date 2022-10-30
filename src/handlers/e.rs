use lazy_static::lazy_static;
use std::collections::HashMap;

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
