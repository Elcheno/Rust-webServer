use crate::models::jwt_model::Claims;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use std::env;

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = env::var("SECRET").expect("JWT SECRET must be set");

    let expiration = get_current_timestamp() + 604800;

    let claims = Claims {
        subject_id: id,
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("SECRET").expect("JWT SECRET must be set");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
