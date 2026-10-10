use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegistrationRequest {
    #[garde(email)]
    #[schema(examples("user@mail.com"))]
    pub email: String,

    #[garde(length(min = 12, max = 128), custom(is_strong_password))]
    #[schema(min_length = 12, max_length = 128, examples("P@ssword123!"))]
    pub password: String,

    #[garde(matches(password))]
    #[schema(min_length = 12, max_length = 128, examples("P@ssword123!"))]
    pub password_confirm: String,
}

fn is_strong_password(password: &str, _ctx: &()) -> garde::Result {
    let has_uppercase = password.chars().any(char::is_uppercase);
    let has_lowercase = password.chars().any(char::is_lowercase);
    let has_digit = password.chars().any(char::is_numeric);
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(garde::Error::new(
            "Must contain an uppercase letter, lowercase letter, number, and special character",
        ));
    }

    Ok(())
}
