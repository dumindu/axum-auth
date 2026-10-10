mod handlers;
mod payload;

use axum::{Router, routing::post};
use utoipa::OpenApi;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handlers::register))
}

#[derive(OpenApi)]
#[openapi(paths(handlers::register), components(schemas(payload::RegistrationRequest)))]
pub struct RegisterApi;
