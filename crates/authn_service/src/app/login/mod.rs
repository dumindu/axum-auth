mod handlers;
mod payload;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handlers::login))
}

#[derive(OpenApi)]
#[openapi(paths(handlers::login), components(schemas(payload::LoginRequest)))]
pub struct LoginApi;
