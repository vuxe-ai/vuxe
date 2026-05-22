use anyhow::Context;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub auth_provider: AuthProvider,
    pub clerk_config: Option<ClerkConfig>,
    pub oidc_config: Option<OidcConfig>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &self.database_url)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("jwt_secret", &"[REDACTED]")
            .field("auth_provider", &self.auth_provider)
            .field("clerk_config", &self.clerk_config)
            .field("oidc_config", &self.oidc_config)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum AuthProvider {
    Clerk,
    Oidc,
}

#[derive(Clone, Debug)]
pub struct ClerkConfig {
    pub jwks_url: String,
    pub authorized_parties: Vec<String>,
}

#[derive(Clone)]
pub struct OidcConfig {
    pub issuer_url: String,
    pub client_id: String,
    #[allow(dead_code)]
    pub client_secret: String,
}

impl std::fmt::Debug for OidcConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OidcConfig")
            .field("issuer_url", &self.issuer_url)
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .finish()
    }
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let auth_provider = match env::var("AUTH_PROVIDER")
            .unwrap_or_else(|_| "clerk".into())
            .to_lowercase()
            .as_str()
        {
            "clerk" => AuthProvider::Clerk,
            "oidc" => AuthProvider::Oidc,
            other => anyhow::bail!("Unknown AUTH_PROVIDER: {other}. Use 'clerk' or 'oidc'."),
        };

        let clerk_config = match auth_provider {
            AuthProvider::Clerk => Some(ClerkConfig {
                jwks_url: env::var("CLERK_JWKS_URL")
                    .unwrap_or_else(|_| "https://api.clerk.com/v1/jwks".into()),
                authorized_parties: env::var("CLERK_AUTHORIZED_PARTIES")
                    .unwrap_or_else(|_| "https://api.clerk.com".into())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            }),
            AuthProvider::Oidc => None,
        };

        let oidc_config = match auth_provider {
            AuthProvider::Oidc => Some(OidcConfig {
                issuer_url: env::var("OIDC_ISSUER_URL")
                    .context("OIDC_ISSUER_URL must be set when AUTH_PROVIDER=oidc")?,
                client_id: env::var("OIDC_CLIENT_ID")
                    .context("OIDC_CLIENT_ID must be set when AUTH_PROVIDER=oidc")?,
                client_secret: env::var("OIDC_CLIENT_SECRET")
                    .context("OIDC_CLIENT_SECRET must be set when AUTH_PROVIDER=oidc")?,
            }),
            AuthProvider::Clerk => None,
        };

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://vuxe:vuxe@localhost:5432/vuxe".into()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .context("PORT must be a valid u16")?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "vuxe-dev-secret-change-in-production".into()),
            auth_provider,
            clerk_config,
            oidc_config,
        })
    }
}
