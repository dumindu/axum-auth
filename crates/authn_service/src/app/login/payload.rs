use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[garde(email)]
    #[schema(examples("user@mail.com"))]
    pub email: String,

    #[garde(length(min = 12, max = 128))]
    #[schema(min_length = 12, max_length = 128, examples("P@ssword123!"))]
    pub password: String,

    #[garde(length(min = 5, max = 255))]
    #[schema(min_length = 5, max_length = 255, examples("iPhone 18 Pro"))]
    pub device_name: String,
}
