extern crate captcha;

use captcha::{Captcha};
use captcha::filters::Noise;

use crate::utils::redis::{self, redis::Commands};


// this crate not very ok :), size not ok!!
pub fn generate() -> Captcha {
    let mut cap = Captcha::new();
    cap.add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(120, 38);
    cap
}

pub fn verify(id: i64) -> bool {
    let mut redis_client = redis::REDIS_POOL.get().unwrap();
    let res = redis_client.get::<i64, String>(id).unwrap();
    if res == "OK" {
        return true
    }
    return false
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate(){
        super::generate();
    }
}
