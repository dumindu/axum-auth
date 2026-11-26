pub mod handler;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler::logout))
}

#[derive(OpenApi)]
#[openapi(paths(handler::logout))]
pub struct Api;
