use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[schema(examples(r#"{"error": "DB_FETCH_FAILED"}"#))]
pub struct ErrorResponse {
    #[expect(dead_code, reason = "OpenAPI YAML")]
    pub error: Error,
}

#[derive(ToSchema)]
pub enum Error {
    // 500
    #[schema(rename = "DB_INSERT_FAILED")]
    DbInsert,
    #[schema(rename = "DB_FETCH_FAILED")]
    DbFetch,
    #[schema(rename = "DB_UPDATE_FAILED")]
    DbUpdate,
    #[schema(rename = "DB_DELETE_FAILED")]
    DbDelete,
    #[schema(rename = "DB_TRANSACTION_BEGIN_FAILED")]
    DbTransactionBegin,
    #[schema(rename = "DB_TRANSACTION_COMMIT_FAILED")]
    DbTransactionCommit,
    #[schema(rename = "PASSWORD_HASH_CONTEXT_FAILED")]
    PasswordHashContext,
    #[schema(rename = "PASSWORD_HASH_ACTION_FAILED")]
    PasswordHashAction,
    #[schema(rename = "JWT_GEN_FAILED")]
    JwtGen,
    #[schema(rename = "TIME_GEN_FAILED")]
    TimeGen,
    // 401
    #[schema(rename = "INVALID_GRANT")]
    InvalidGrant,
    #[schema(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[schema(rename = "INVALID_REFRESH_TOKEN")]
    InvalidRefreshToken,
    // 401: jwt auth
    #[schema(rename = "INVALID_TOKEN")]
    InvalidToken,
    #[schema(rename = "INVALID_TOKEN_ISSUER")]
    InvalidTokenIssuer,
    #[schema(rename = "INVALID_TOKEN_AUDIENCE")]
    InvalidTokenAudience,
    #[schema(rename = "NOT_YET_VALID_TOKEN")]
    NotYetValidToken,
    #[schema(rename = "EXPIRED_TOKEN")]
    ExpiredToken,
    // 403
    #[serde(skip)]
    Forbidden,
    // 404
    #[serde(skip)]
    NotFound,
    // 409
    #[schema(rename = "DUPLICATE_ENTITY")]
    DuplicateEntity,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, bytes): (StatusCode, &'static [u8]) = match self {
            // 500
            Error::DbInsert => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_INSERT_FAILED\"}")
            }
            Error::DbFetch => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_FETCH_FAILED\"}")
            }
            Error::DbUpdate => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_UPDATE_FAILED\"}")
            }
            Error::DbDelete => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_DELETE_FAILED\"}")
            }
            Error::DbTransactionBegin => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"DB_TRANSACTION_BEGIN_FAILED\"}")
            }
            Error::DbTransactionCommit => (
                StatusCode::INTERNAL_SERVER_ERROR,
                b"{\"error\": \"DB_TRANSACTION_COMMIT_FAILED\"}",
            ),
            Error::PasswordHashContext => (
                StatusCode::INTERNAL_SERVER_ERROR,
                b"{\"error\": \"PASSWORD_HASH_CONTEXT_FAILED\"}",
            ),
            Error::PasswordHashAction => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"PASSWORD_HASH_ACTION_FAILED\"}")
            }
            Error::JwtGen => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"Jwt_GEN_FAILED\"}")
            }
            Error::TimeGen => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"{\"error\": \"TIME_GEN_FAILED\"}")
            }
            // 401
            Error::InvalidGrant => (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_GRANT\"}"),
            Error::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_CREDENTIALS\"}")
            }
            Error::InvalidRefreshToken => {
                (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_REFRESH_TOKEN\"}")
            }
            // 401: jwt auth
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_TOKEN\"}"),
            Error::InvalidTokenIssuer => {
                (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_TOKEN_ISSUER\"}")
            }
            Error::InvalidTokenAudience => {
                (StatusCode::UNAUTHORIZED, b"{\"error\": \"INVALID_TOKEN_AUDIENCE\"}")
            }
            Error::NotYetValidToken => {
                (StatusCode::UNAUTHORIZED, b"{\"error\": \"NOT_YET_VALID_TOKEN\"}")
            }
            Error::ExpiredToken => (StatusCode::UNAUTHORIZED, b"{\"error\": \"EXPIRED_TOKEN\"}"),
            // 403
            Error::Forbidden => (StatusCode::FORBIDDEN, b""),
            // 404
            Error::NotFound => (StatusCode::NOT_FOUND, b""),
            // 409
            Error::DuplicateEntity => (StatusCode::CONFLICT, b"{\"error\": \"DUPLICATE_ENTITY\"}"),
        };

        let mut response = Response::new(Body::from(bytes));
        *response.status_mut() = status;
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

        response
    }
}
