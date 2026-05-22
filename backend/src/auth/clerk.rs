use crate::config::ClerkConfig;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClerkClaims {
    pub sub: String,
    #[allow(dead_code)]
    #[serde(rename = "azp")]
    pub authorized_party: Option<String>,
}

pub struct ClerkVerifier {
    config: ClerkConfig,
}

impl ClerkVerifier {
    pub fn new(config: ClerkConfig) -> Self {
        Self { config }
    }

    /// Verify a Clerk session JWT and return the user's external ID (sub).
    pub async fn verify(&self, token: &str) -> anyhow::Result<String> {
        // Fetch JWKS
        let jwks: jsonwebtoken::jwk::JwkSet =
            reqwest::get(&self.config.jwks_url).await?.json().await?;

        // Decode header to get the key ID
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("JWT missing kid header"))?;

        // Find the matching key in the JWKS
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| anyhow::anyhow!("No matching JWK found for kid: {kid}"))?;

        let decoding_key = DecodingKey::from_jwk(&jwk)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&self.config.authorized_parties);
        validation.validate_exp = true;

        let token_data = decode::<ClerkClaims>(token, &decoding_key, &validation)?;

        Ok(token_data.claims.sub)
    }
}
