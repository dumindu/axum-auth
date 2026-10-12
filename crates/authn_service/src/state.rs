use toasty::Db;

use crate::{JwtTokenConf, SecretsConf, ServerConf};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub server_conf: ServerConf,
    pub secrets_conf: SecretsConf,
    pub jwt_token_conf: JwtTokenConf,
}
