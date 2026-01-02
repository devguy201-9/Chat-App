use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

pub fn encode_jwt(user_id: Uuid) -> Result<String> {
    let claims = Claims {
        sub: String::from("Login Token"),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        user_id,
    };

    let secret =
        env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".to_string()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(token)
}

pub fn decode_jwt(token: String) -> Result<Uuid> {
    let secret =
        env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".to_string()))?;

    let Claims {
        user_id,
        sub: _,
        exp: _,
    } = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| Error::DecodeJwtFailed(e.to_string()))?
    .claims;

    Ok(user_id)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // the subject of the token
    pub exp: usize,  // the expiry time
    pub user_id: Uuid,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use uuid::Uuid;

    #[test]
    fn encode_decode_roundtrip() {
        // set test secret
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-for-ci");
        }
        let id = Uuid::new_v4();
        let token = encode_jwt(id).expect("encode should succeed");
        let decoded = decode_jwt(token).expect("decode should succeed");
        assert_eq!(id, decoded);
    }

    #[test]
    fn missing_secret_returns_error() {
        unsafe {
            std::env::remove_var("JWT_SECRET");
        }
        let id = Uuid::new_v4();
        let err = encode_jwt(id).err().expect("should error");
        match err {
            crate::error::Error::EnvVarNotFound(_) => {}
            _ => panic!("expected EnvVarNotFound"),
        }
    }
}
