use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use jiff::{SignedDuration, Timestamp};
use tracing::error;
use uuid::Uuid;

use crate::{
    AppState,
    app::sha_hashes,
    errors::{Error, Error::TimeGen},
    models::OauthChallenge,
};

const PROVIDER: &str = "google";
const FLOW_TYPE: &str = "login";
const CHALLENGE_LIFETIME: SignedDuration = SignedDuration::from_mins(5);

#[utoipa::path(post, path = "/v1/google/login", tag = "auth")]
pub async fn login_with_google(
    State(mut state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let (verifier, challenge) =
        sha_hashes::pkce_code_verifier_and_challenge().map_err(|_| Error::PkceGen)?;

    let expires_at = Timestamp::now().checked_add(CHALLENGE_LIFETIME).map_err(|_| TimeGen)?;

    let (state_uuid, nonce_uuid) = (Uuid::new_v4(), Uuid::new_v4());
    let _saved = toasty::create!(OauthChallenge {
        state: state_uuid,
        nonce: Some(nonce_uuid),
        code_verifier: verifier,
        provider: PROVIDER,
        flow_type: FLOW_TYPE,
        expires_at
    })
    .exec(&mut state.db)
    .await
    .map_err(|error| {
        error!(target: "db", %error, "db insert failed");
        Error::DbInsert
    })?;

    let (state_string, nonce_string) = (state_uuid.to_string(), nonce_uuid.to_string());
    let url = state.google_oauth2.authorization_url(&state_string, &nonce_string, &challenge).await;

    Ok(Redirect::to(url.as_str()))
}
