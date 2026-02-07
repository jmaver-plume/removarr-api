use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

const ACCESS_TOKEN_DURATION_MINUTES: i64 = 15;
const REFRESH_TOKEN_DURATION_DAYS: i64 = 30;
const JWT_SECRET: &str = "your-secret-key-change-this-in-production"; // TODO: Move to config

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // username
    pub admin_id: i32,
    pub exp: i64,     // expiration
    pub iat: i64,     // issued at
}

pub fn create_access_token(admin_id: i32, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = (now + Duration::minutes(ACCESS_TOKEN_DURATION_MINUTES)).timestamp();
    
    let claims = Claims {
        sub: username.to_string(),
        admin_id,
        exp,
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

pub fn create_refresh_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn get_refresh_token_expiry() -> chrono::NaiveDateTime {
    (Utc::now() + Duration::days(REFRESH_TOKEN_DURATION_DAYS)).naive_utc()
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn generate_random_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
