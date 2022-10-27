extern crate captcha;

use std::path::Path;
use captcha::{Captcha, RngCaptcha};
use captcha::filters::Noise;


pub fn generate() -> Captcha {
    let mut cap = Captcha::new();
    cap.add_chars(4)
        .apply_filter(Noise::new(0.1))
        .view(220, 120);
    cap.save(Path::new("captcha.png")).expect("save failed");
    cap
}

pub fn verify() -> bool {
    return true
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate(){
        super::generate();
    }
}
