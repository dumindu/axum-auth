use std::{collections::HashSet, convert::Infallible, str::FromStr};

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub server: ServerConf,

    #[envconfig(nested)]
    pub db: DbConf,

    #[envconfig(nested)]
    pub cache: CacheConf,

    #[envconfig(nested)]
    pub nats: NatsConf,

    #[envconfig(nested)]
    pub secrets: SecretsConf,

    #[envconfig(nested)]
    pub oauth2: OAuth2Conf,

    #[envconfig(nested)]
    pub http_client: HttpClientConf,

    #[envconfig(nested)]
    pub jwt: JwtTokenConf,

    #[envconfig(nested)]
    pub cookies: CookiesConf,
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
    pub password: SecretString,
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
            self.protocol,
            self.user,
            self.password.expose_str(),
            self.host,
            self.port,
            self.db_name
        )
    }
}

#[derive(Clone, Envconfig)]
pub struct CacheConf {
    #[envconfig(from = "CACHE_HOST")]
    pub host: String,
    #[envconfig(from = "CACHE_PORT", default = "6379")]
    pub port: u16,
}

impl CacheConf {
    pub fn to_database_url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}

#[derive(Clone, Envconfig)]
pub struct NatsConf {
    #[envconfig(from = "NATS_HOST")]
    pub host: String,
    #[envconfig(from = "NATS_PORT", default = "4222")]
    pub port: u16,
    #[envconfig(from = "NATS_USER_PRIVATE_KEY")]
    pub user_private_key: SecretString,
}

impl NatsConf {
    pub fn to_addr(&self) -> String {
        format!("nats://{}:{}/", self.host, self.port)
    }
}

#[derive(Clone, Envconfig)]
pub struct SecretsConf {
    #[envconfig(from = "PASSWORD_PEPPER")]
    pub password_pepper: SecretString,
    #[envconfig(from = "DEVICE_ID_HASH_SALT")]
    pub device_id_hash_salt: SecretString,
}

#[derive(Envconfig)]
pub struct OAuth2Conf {
    #[envconfig(from = "REDIRECT_HOST")]
    pub redirect_host: String,

    #[envconfig(nested)]
    pub google: GoogleOAuth2Conf,
}

#[derive(Envconfig)]
pub struct HttpClientConf {
    #[envconfig(from = "HTTP_CLIENT_MAX_IDLE_PER_HOST")]
    pub max_idle_per_host: usize,
    #[envconfig(from = "HTTP_CLIENT_POOL_IDLE_TIMEOUT_IN_SECS", default = "90")]
    pub pool_idle_timeout_in_secs: u64,
    #[envconfig(from = "HTTP_CLIENT_CONNECT_TIMEOUT_IN_SECS")]
    pub connect_timeout_in_secs: u64,
    #[envconfig(from = "HTTP_CLIENT_TIMEOUT_IN_SECS")]
    pub timeout_in_secs: u64,
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
    pub audience: StringSet,
    #[envconfig(from = "JWT_TOKEN_LIFETIME_IN_MINS")]
    pub lifetime_in_mins: i64,
}

#[derive(Clone, Envconfig)]
pub struct CookiesConf {
    #[envconfig(from = "COOKIE_DOMAIN", default = "localhost")]
    pub domain: String,
    #[envconfig(from = "DEVICE_ID_COOKIE_LIFETIME_IN_DAYS", default = "7")]
    pub device_id_lifetime_in_days: i64,
    #[envconfig(from = "REFRESH_TOKEN_COOKIE_LIFETIME_IN_DAYS", default = "30")]
    pub refresh_token_lifetime_in_days: i64,
}

#[derive(Clone, Envconfig)]
pub struct GoogleOAuth2Conf {
    #[envconfig(from = "GOOGLE_OAUTH2_CLIENT_ID")]
    pub client_id: SecretString,
    #[envconfig(from = "GOOGLE_OAUTH2_CLIENT_SECRET")]
    pub client_secret: SecretString,
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

#[derive(Clone)]
pub struct StringSet(pub HashSet<String>);

impl FromStr for StringSet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();

        Ok(StringSet(items))
    }
}
