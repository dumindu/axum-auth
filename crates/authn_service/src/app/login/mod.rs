mod handler;
mod payload;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler::login))
}

#[derive(OpenApi)]
#[openapi(paths(handler::login), components(schemas(payload::LoginRequest)))]
pub struct Api;
