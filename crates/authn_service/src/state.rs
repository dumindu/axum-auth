use async_nats::jetstream::Context;
use redis::aio::MultiplexedConnection;
use toasty::Db;

use crate::{CookiesConf, JwtTokenConf, SecretsConf, ServerConf};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cache: MultiplexedConnection,
    pub js: Context,
    pub server_conf: ServerConf,
    pub secrets_conf: SecretsConf,
    pub jwt_token_conf: JwtTokenConf,
    pub cookies_conf: CookiesConf,
}
