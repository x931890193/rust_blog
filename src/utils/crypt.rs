use base64;
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha2::Sha256;

pub fn sha256(src: &str) -> String {
    let mut m = Sha256::new();
    m.input(src.as_bytes());
    m.result_str()
}

pub fn b64_decode(src: &str) -> Result<String, base64::DecodeError> {
    let res = base64::decode(src);
    return match res {
        Ok(res) => {
            if let Ok(str_res) = String::from_utf8(res) {
                Ok(str_res)
            } else {
                Err(base64::DecodeError::InvalidLength)
            }
        }
        Err(e) => Err(e),
    };
}

pub fn b64_encode(src: &str) -> String {
    base64::encode(src)
}

pub fn md5(src: &str) -> String {
    let mut src_code = Md5::new();
    src_code.input(src.as_bytes());
    src_code.result_str()
}

#[cfg(test)]
mod test {
    use crate::utils::crypt::md5;
    use crate::utils::crypt::sha256;
    #[test]
    fn test_sha256() {
        println!("{}", sha256("1111"))
    }
    #[test]
    fn test_md5() {
        println!("{}", md5("1111"))
    }
}
