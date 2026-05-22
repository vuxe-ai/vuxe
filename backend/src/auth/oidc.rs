use crate::config::OidcConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OidcDiscovery {
    jwks_uri: String,
    issuer: String,
}

#[derive(Debug, Deserialize)]
pub struct OidcClaims {
    pub sub: String,
    #[allow(dead_code)]
    pub iss: String,
    #[allow(dead_code)]
    pub aud: Option<serde_json::Value>,
}

pub struct OidcVerifier {
    config: OidcConfig,
}

impl OidcVerifier {
    pub fn new(config: OidcConfig) -> Self {
        Self { config }
    }

    /// Discover the OIDC provider's JWKS URI from the well-known endpoint,
    /// then verify the JWT and return the user's external ID (sub).
    pub async fn verify(&self, token: &str) -> anyhow::Result<String> {
        let discovery_url = format!(
            "{}/.well-known/openid-configuration",
            self.config.issuer_url.trim_end_matches('/')
        );

        let discovery: OidcDiscovery = reqwest::get(&discovery_url)
            .await?
            .json()
            .await?;

        // Fetch JWKS
        let jwks: jsonwebtoken::jwk::JwkSet =
            reqwest::get(&discovery.jwks_uri).await?.json().await?;

        let header = jsonwebtoken::decode_header(token)?;
        let kid = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("JWT missing kid header"))?;

        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| anyhow::anyhow!("No matching JWK found for kid: {kid}"))?;

        let decoding_key = jsonwebtoken::DecodingKey::from_jwk(&jwk)?;

        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[&discovery.issuer]);
        validation.set_audience(&[&self.config.client_id]);
        validation.validate_exp = true;

        let token_data =
            jsonwebtoken::decode::<OidcClaims>(token, &decoding_key, &validation)?;

        Ok(token_data.claims.sub)
    }
}
