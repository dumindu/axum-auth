pub mod google {
    use reqwest::{
        Client as HttpClient, Url,
        header::{CONTENT_TYPE, HeaderValue},
    };

    use crate::{GoogleOAuth2Conf, config::SecretString};

    const FORM_HEADER: HeaderValue = HeaderValue::from_static("application/x-www-form-urlencoded");

    #[derive(Clone)]
    struct Config {
        client_id: SecretString,
        client_secret: SecretString,
        redirect_uri: String,
    }

    #[derive(Clone)]
    pub struct Client {
        http: HttpClient,
        config: Config,
        auth_url: Url,
        token_url: Url,
        revoke_url: Url,
    }

    impl Client {
        pub fn new(conf: &GoogleOAuth2Conf, redirect_host: &str, http: HttpClient) -> Self {
            let config = Config {
                client_id: conf.client_id.clone(),
                client_secret: conf.client_secret.clone(),
                redirect_uri: format!("{}/v1/google/login/callback", redirect_host),
            };

            let mut auth_url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth").unwrap();
            let token_url = Url::parse("https://oauth2.googleapis.com/token").unwrap();
            let revoke_url = Url::parse("https://oauth2.googleapis.com/revoke").unwrap();

            auth_url
                .query_pairs_mut()
                .append_pair("client_id", config.client_id.expose_str())
                .append_pair("access_type", "offline")
                .append_pair("scope", "openid email")
                .append_pair("response_type", "code")
                .append_pair("code_challenge_method", "S256")
                .append_pair("redirect_uri", &config.redirect_uri);

            Self { http, config, auth_url, token_url, revoke_url }
        }

        pub async fn authorization_url(&self, state: &str, nonce: &str, challenge: &str) -> Url {
            let mut url = self.auth_url.clone();
            url.query_pairs_mut()
                .append_pair("state", state)
                .append_pair("nonce", nonce)
                .append_pair("code_challenge", challenge);
            url
        }

        pub async fn exchange_code(&self, code: &str, code_verifier: &str) -> anyhow::Result<()> {
            let body = [
                ("client_id", self.config.client_id.expose_str()),
                ("client_secret", self.config.client_secret.expose_str()),
                ("code", code),
                ("code_verifier", code_verifier),
                ("grant_type", "authorization_code"),
                ("redirect_uri", &self.config.redirect_uri),
            ];

            let resp = self
                .http
                .post(self.token_url.clone())
                .header(CONTENT_TYPE, FORM_HEADER)
                .form(&body)
                .send()
                .await?;

            resp.error_for_status()?;

            Ok(())
        }
    }
}
