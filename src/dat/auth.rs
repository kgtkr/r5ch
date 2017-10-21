use utils;
use crypto::hmac::Hmac;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crypto::mac::Mac;
use rustc_serialize::hex::ToHex;

pub struct AuthClient {
  x2ch_ua: String,
  app_key: String,
  ronin: Option<utils::Ronin>,
  ct: String,
}

const HM_KEY: &str = "DgQ3aNpoluV1cl3GFJAqitBg5xKiXZ";


impl AuthClient {
  pub fn new(x2ch_ua: String, app_key: String, ronin: Option<Ronin>) -> AuthClient {
    AuthClient {
      x2ch_ua: x2ch_ua,
      app_key: app_key,
      ronin: ronin,
      ct: "1234567890".to_string(),
    }
  }

  fn get_hb(&self) -> String {
    let message = (&self.app_key).to_string() + &self.ct;
    let mut hmac = Hmac::new(Sha256::new(), HM_KEY.as_bytes());
    hmac.input(message.as_bytes());
    hmac.result().code().to_hex()
  }
}
