use utoipa::{OpenApi, openapi::OpenApi as OpenApiDoc};

#[derive(OpenApi)]
#[openapi(
    info(title = "Authn Service API", version = "1.0.0"),
    servers(
        (url = "http://localhost:3000", description = "Development")
    ),
    components(schemas(
        crate::errors::ErrorResponse,
        crate::app::ValidationErrorResponse
    )),
)]
pub struct ApiDoc;

pub fn build_api_doc() -> OpenApiDoc {
    let mut openapi = ApiDoc::openapi();
    openapi.merge(crate::app::register::RegisterApi::openapi());
    openapi
}
