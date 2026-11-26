use async_nats::jetstream::Context;
use redis::aio::MultiplexedConnection;
use toasty::Db;

use crate::{CookiesConf, GoogleOAuth2Client, JwtTokenConf, SecretsConf, ServerConf};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cache: MultiplexedConnection,
    pub js: Context,
    pub google_oauth2: GoogleOAuth2Client,
    pub server_conf: ServerConf,
    pub secrets_conf: SecretsConf,
    pub jwt_token_conf: JwtTokenConf,
    pub cookies_conf: CookiesConf,
}
