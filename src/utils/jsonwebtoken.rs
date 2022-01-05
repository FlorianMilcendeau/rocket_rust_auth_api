use jsonwebtoken::{encode, errors::Error, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub name: String,
    pub email: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: i32, name: &String, email: &String) -> Self {
        Self {
            sub,
            name: name.to_owned(),
            email: email.to_owned(),
            exp: 1000000,
        }
    }
}

pub fn generate_json_web_token(claims: Claims) -> Result<String, Error> {
    let token = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(include_bytes!("../../id_rsa_priv.pem"))?,
    );

    return token;
}
