use std::{convert::Infallible, str::FromStr};

use envconfig::Envconfig;
use jiff::SignedDuration;

#[derive(Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub server: ServerConf,

    #[envconfig(nested)]
    pub db: DbConf,

    #[envconfig(nested)]
    pub secrets: SecretsConf,

    #[envconfig(nested)]
    pub jwt: JwtTokenConf,
}

impl AppConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration! Check the .env file.")
    }
}

#[derive(Clone, Envconfig)]
pub struct ServerConf {
    #[envconfig(from = "SERVER_PORT", default = "3000")]
    pub port: u16,
    #[envconfig(from = "SERVER_ALLOWED_ORIGINS")]
    pub allowed_origins: String,
    #[envconfig(from = "SERVER_ALLOWED_METHODS")]
    pub allowed_methods: String,
    #[envconfig(from = "SERVER_ALLOWED_HEADERS")]
    pub allowed_headers: String,
    #[envconfig(from = "SERVER_DEFAULT_BODY_LIMIT", default = "1048576")]
    pub default_body_limit: usize,
}

impl ServerConf {
    pub fn to_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

#[derive(Envconfig)]
pub struct DbConf {
    #[envconfig(from = "DB_PROTOCOL", default = "postgres")]
    pub protocol: String,
    #[envconfig(from = "DB_HOST")]
    pub host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,
    #[envconfig(from = "DB_USER")]
    pub user: String,
    #[envconfig(from = "DB_PASS")]
    pub password: String,
    #[envconfig(from = "DB_NAME")]
    pub db_name: String,
}

impl DbConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration! Check the .env file.")
    }

    pub fn to_database_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.protocol, self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

#[derive(Clone, Envconfig)]
pub struct SecretsConf {
    #[envconfig(from = "PASSWORD_PEPPER")]
    pub password_pepper: SecretString,
    #[envconfig(from = "DEVICE_ID_HASH_SALT")]
    pub device_id_hash_salt: SecretString,
}

#[derive(Clone, Envconfig)]
pub struct JwtTokenConf {
    #[envconfig(from = "JWT_TOKEN_PRIVATE_KEY")]
    pub private_key: SecretString,
    #[envconfig(from = "JWT_TOKEN_PUBLIC_KEY")]
    pub public_key: SecretString,
    #[envconfig(from = "JWT_TOKEN_ISSUER")]
    pub issuer: String,
    #[envconfig(from = "JWT_TOKEN_AUDIENCE")]
    pub audience: StringVec,
    #[envconfig(from = "JWT_TOKEN_LIFETIME")]
    pub lifetime: Duration,
}

// Custom type: SecretString
#[derive(Clone)]
pub struct SecretString(String);

impl FromStr for SecretString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SecretString(s.to_string()))
    }
}

impl SecretString {
    pub fn expose_str(&self) -> &str {
        &self.0
    }
}

// Custom type: StringArray
#[derive(Clone)]
pub struct StringVec(pub Vec<String>);

impl FromStr for StringVec {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();
        Ok(StringVec(items))
    }
}

// Custom type: Duration
#[derive(Clone)]
pub struct Duration(pub SignedDuration);

impl FromStr for Duration {
    type Err = jiff::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<SignedDuration>()?;
        Ok(Duration(parsed))
    }
}
