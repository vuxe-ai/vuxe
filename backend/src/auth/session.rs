use serde::{Deserialize, Serialize};

/// Claims embedded in the session JWT returned by /auth/login.
/// Used both for encoding (in auth routes) and decoding (in middleware).
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub sub: String, // user UUID
    pub ext: String, // external ID from auth provider
    pub iat: usize,
    pub exp: usize,
}
