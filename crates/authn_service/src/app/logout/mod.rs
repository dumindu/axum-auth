pub mod handlers;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handlers::logout))
}

#[derive(OpenApi)]
#[openapi(paths(handlers::logout))]
pub struct LogoutApi;
