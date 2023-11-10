use std::time::Duration;

use once_cell::sync::Lazy;
use poem_openapi::{param::Query, ApiResponse, OpenApi};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sqlx::{Pool, Postgres};

mod google;
mod microsoft;

pub struct AuthApi {
    pool: Pool<Postgres>,
}

impl AuthApi {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

/// The API to logging in
#[OpenApi]
impl AuthApi {
    #[oai(path = "/google", method = "get")]
    async fn google_redirect(&self) -> OAuthRedirectResponse {
        static GOOGLE_REDIRECT: Lazy<String> = Lazy::new(AuthApi::google_redirect_string);
        OAuthRedirectResponse::SuccessfulRedirect(GOOGLE_REDIRECT.clone())
    }

    #[oai(path = "/google/callback", method = "get")]
    async fn google_callback_req(
        &self,
        code: Query<String>,
        #[oai(name = "scope")] _scope: Query<String>,
        #[oai(name = "authuser")] _authuser: Query<String>,
        #[oai(name = "prompt")] _prompt: Query<String>,
    ) -> OAuthCallbackResponse {
        self.google_callback(code.0).await
    }

    #[oai(path = "/microsoft", method = "get")]
    async fn microsoft_redirect(&self) -> OAuthRedirectResponse {
        static MICROSOFT_REDIRECT: Lazy<String> = Lazy::new(AuthApi::microsoft_redirect_string);
        OAuthRedirectResponse::SuccessfulRedirect(MICROSOFT_REDIRECT.clone())
    }

    #[oai(path = "/microsoft/callback", method = "get")]
    async fn ms_callback_req(
        &self,
        code: Query<String>,
        #[oai(name = "session_state")] _session_state: Query<String>,
    ) -> OAuthCallbackResponse {
        self.ms_callback(code.0).await
    }
}

#[derive(ApiResponse)]
pub enum OAuthCallbackResponse {
    /// When everything goes right and the user successfully authenticates themselves
    #[oai(status = "301")]
    SuccessfullyAuthenticated(#[oai(header = "Location")] String),
    /// When something went wrong during authentication on the server side
    #[oai(status = "500")]
    AuthenticationError,
}

#[derive(ApiResponse)]
pub enum OAuthRedirectResponse {
    #[oai(status = "302")]
    SuccessfulRedirect(#[oai(header = "Location")] String),
}
