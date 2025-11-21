use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, errors::ErrorKind};
use chrono::{Utc, Duration};
use std::env;
use log::error;

const JWT_SECRET_KEY: &str = "AUTH_TOKEN";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_token(user_identifier: &str) -> Result<String, String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(24);

    let claims = Claims {
        sub: user_identifier.to_string(),
        iat: now.timestamp(),
        exp: expiration.timestamp(),
    };

    let secret = env::var(JWT_SECRET_KEY).map_err(|_| "JWT Secret no configurado".to_string())?;

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Error al codificar JWT: {}", e)),
    }
}

pub fn validate_token(token: &str) -> Result<Claims, String> {
    let secret = env::var(JWT_SECRET_KEY).map_err(|_| "JWT Secret no configurado".to_string())?;

    let mut validation = Validation::default();
    validation.validate_aud = false;
    validation.validate_exp = true;

    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(data) => Ok(data.claims),
        Err(err) => {
            match err.kind() {
                ErrorKind::ExpiredSignature => Err("Token expirado.".to_string()),
                _ => {
                    error!("JWT Decode Error: {:?}", err);
                    Err("Token inválido o mal formado.".to_string())
                }
            }
        }
    }
}