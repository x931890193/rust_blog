extern crate captcha;

use captcha::filters::Noise;
use captcha::Captcha;

use crate::utils::cache::{self, redis::Commands};

// this crate not very ok :), size not ok!!
pub fn generate() -> Captcha {
    let mut cap = Captcha::new();
    cap.add_chars(5).apply_filter(Noise::new(0.1)).view(120, 38);
    cap
}

pub fn verify(id: i64, code: String) -> bool {
    let mut redis_client = cache::REDIS_POOL.get().unwrap();
    let res = redis_client.get::<i64, String>(id).unwrap();
    return res.to_lowercase() == code.to_lowercase();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate() {
        super::generate();
    }

    #[test]
    fn test_redis() {
        use crate::utils::cache::{self, redis::Commands};
        let mut redis_client = cache::REDIS_POOL.get().unwrap();
        let res = redis_client.get::<i64, String>(11).unwrap();
        println!("res: {}", res)
    }
}
