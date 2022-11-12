use std::fs::read_to_string;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;


#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    pub host: String,
    pub port: String,
    pub pg_port: String,
    pub user: String,
    pub pg_user: String,
    pub password: String,
    pub db: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub host: String,
    pub port: String,
    pub db: i32,
    pub user: Option<String>,
    pub pass_word: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QiNiu {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mail {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub max_client: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AliPay {
    pub private_key: String,
    pub public_key: String,
    pub app_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WechatPay {
    pub app_id: String,
    pub app_secret: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub db: Db,
    pub cache: Cache,
    pub qiniu: QiNiu,
    pub github: GitHub,
    pub mail: Mail,
    pub ali_pay: AliPay,
    pub wechat_pay: WechatPay
}

lazy_static!(
    pub static ref CONFIGURATION: Config = {
        let yaml_str = read_to_string("./.config.yml").unwrap();
        let conf: Config = serde_yaml::from_str(&yaml_str).unwrap();
        conf
    };
);

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use super::Config;
    #[test]
    fn test_yaml_read() {
        let yaml_str = read_to_string("./.config.yml").unwrap();
        let conf: Config = serde_yaml::from_str(&yaml_str).unwrap();
        println!("{:?}", conf.ali_pay)
    }
}

