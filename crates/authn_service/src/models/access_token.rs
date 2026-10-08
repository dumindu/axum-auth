use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AccessTokenResponse {
    pub access_token: String,
}
