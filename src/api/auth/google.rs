use std::time::Duration;

use dotenv_codegen::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::api::auth::OAuthCallbackResponse;

use super::AuthApi;

impl AuthApi {
    const GOOGLE: GoogleOauthConst = GoogleOauthConst {
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
        token_url: "https://accounts.google.com/o/oauth2/token",
        id: dotenv!("GOOGLE_CLIENT_ID"),
        secret: dotenv!("GOOGLE_CLIENT_SECRET"),
        callback_url: "http://localhost:3000/api/sessions/oauth/google/callback",
    };

    pub fn google_redirect_string() -> String {
        type Api = AuthApi;
        #[derive(Serialize)]
        struct GoogleRequestOptions {
            redirect_uri: String,
            client_id: String,
            access_type: String,
            response_type: String,
            prompt: String,
            scope: String,
        }

        let options = GoogleRequestOptions {
                redirect_uri: Api::GOOGLE.callback_url.to_string(),
                client_id: Api::GOOGLE.id.to_string(),
                access_type: "offline".to_string(),
                response_type: "code".to_string(),
                prompt: "consent".to_string(),
                scope: "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email".to_string()
            };

        let serialized = serde_qs::to_string(&options).expect("It decided to fail");
        format!("{}?{serialized}", Api::GOOGLE.auth_url)
    }

    pub(super) async fn request_google_user(
        &self,
        root: &str,
        query: String,
    ) -> OAuthCallbackResponse {
        let client = Client::new();
        let req = client
            .post(format!("{root}?{query}"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query);

        let res = match req.timeout(Duration::from_secs(10)).send().await {
            Ok(it) => it,
            Err(err) => {
                println!(": {}", err);
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        match res.error_for_status_ref() {
            Ok(_res) => (),
            Err(err) => {
                println!("Error: {} Body: {}", err, res.text().await.unwrap());
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        let text = match res.text().await {
            Ok(it) => it,
            Err(err) => {
                println!("Error when getting body: {}", err);
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        let data: GoogleAuthResponse = match from_str(&text) {
            Ok(it) => it,
            Err(err) => {
                return OAuthCallbackResponse::AuthenticationError;
            }
        };

        OAuthCallbackResponse::SuccessfullyAuthenticated("http://localhost:3000/".to_string())
    }
    pub(crate) async fn google_callback(&self, code: String) -> OAuthCallbackResponse {
        #[derive(Serialize)]
        struct GoogleCallbackMessage {
            code: String,
            client_id: String,
            client_secret: String,
            redirect_uri: String,
            grant_type: String,
            // response_type: String
        }

        let to_send = GoogleCallbackMessage {
            code,
            client_id: Self::GOOGLE.id.to_string(),
            client_secret: Self::GOOGLE.secret.to_string(),
            redirect_uri: Self::GOOGLE.callback_url.to_string(),
            grant_type: "authorization_code".to_string(),
        };

        let serialized = serde_qs::to_string(&to_send).unwrap();
        self.request_google_user(Self::GOOGLE.token_url, serialized)
            .await
    }
}

struct GoogleOauthConst {
    auth_url: &'static str,
    token_url: &'static str,
    id: &'static str,
    secret: &'static str,
    callback_url: &'static str,
}

#[derive(Deserialize)]
pub struct GoogleAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_in: String,
    pub scope: String,
    pub token_type: String,
    #[serde(rename = "id_token")]
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
