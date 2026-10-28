use std::{convert::Infallible, str::FromStr};

use envconfig::Envconfig;
use jiff::SignedDuration;

#[derive(Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub nats: NatsConf,

    #[envconfig(nested)]
    pub smtp: SmtpConf,
}

impl AppConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration! Check the .env file.")
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

#[derive(Envconfig)]
pub struct SmtpConf {
    #[envconfig(from = "SMTP_HOST")]
    pub host: String,
    #[envconfig(from = "SMTP_PORT", default = "1025")]
    pub port: u16,
    #[envconfig(from = "SMTP_USER")]
    pub user: String,
    #[envconfig(from = "SMTP_PASS")]
    pub password: SecretString,
    #[envconfig(from = "SMTP_MAX_CONN", default = "10")]
    pub max_connection: u32,
    #[envconfig(from = "SMTP_MIN_CONN", default = "1")]
    pub min_connection: u32,
    #[envconfig(from = "SMTP_TIMEOUT", default = "5s")]
    pub timeout: Duration,
}

impl SmtpConf {
    pub fn to_addr(&self) -> String {
        format!("smtps://{}:{}@{}:{}", self.user, self.password.expose_str(), self.host, self.port)
    }
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
