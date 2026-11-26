mod handler;

use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler::login_with_google))
}

#[derive(OpenApi)]
#[openapi(paths(handler::login_with_google))]
pub struct Api;
