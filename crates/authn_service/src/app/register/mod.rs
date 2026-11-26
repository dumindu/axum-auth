mod handler;
mod payload;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler::register))
}

#[derive(OpenApi)]
#[openapi(paths(handler::register), components(schemas(payload::RegistrationRequest)))]
pub struct Api;
