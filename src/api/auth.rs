use std::time::Duration;

use dotenv_codegen::dotenv;
use jsonwebtoken::DecodingKey;
use poem_openapi::{param::Query, ApiResponse, OpenApi};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sqlx::{query, Pool, Postgres};

use crate::db::login_user;

pub struct AuthApi {
    pool: Pool<Postgres>,
}

impl AuthApi {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[OpenApi]
impl AuthApi {
    const GOOGLE_ID: &str = dotenv!("GOOGLE_CLIENT_ID");
    const GOOGLE_SECRET: &str = dotenv!("GOOGLE_CLIENT_SECRET");

    #[oai(path = "/oauth/google", method = "get")]
    async fn google(&self) -> GoogleAuthRedirect {
        const ROOT_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
        let options = OAuthRequestOptions {
            redirect_uri: "http://localhost:3000/api/sessions/oauth/google/callback".to_string(),
            client_id: Self::GOOGLE_ID.to_string(),
            access_type: "offline".to_string(),
            response_type: "code".to_string(),
            prompt: "consent".to_string(),
            // Because serde_qs uses dum dum bracket arrays and not +
            scope: vec![
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ]
            .join(" "),
        };

        let serialized = serde_qs::to_string(&options).expect("It decided to fail");

        // To prevent caching
        GoogleAuthRedirect::SuccessfulRedirect(format!("{ROOT_URL}?{serialized}"))
    }
    #[oai(path = "/oauth/google/callback", method = "get")]
    async fn google_callback(
        &self,
        code: Query<String>,
        scope: Query<String>,
        authuser: Query<String>,
        prompt: Query<String>,
    ) -> OAuthCallback {
        let client = Client::new();
        const ROOT_URL: &str = "https://accounts.google.com/o/oauth2/token";

        let to_send = OAuthCallbackOptions {
            code: code.0,
            client_id: Self::GOOGLE_ID.to_string(),
            client_secret: Self::GOOGLE_SECRET.to_string(),
            redirect_uri: "http://localhost:3000/api/sessions/oauth/google/callback".to_string(),
            grant_type: "authorization_code".to_string(),
        };

        let serialized = serde_qs::to_string(&to_send).expect("");

        let req = client
            .post(format!("{ROOT_URL}?{serialized}"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serialized);

        let res = match req.timeout(Duration::from_secs(5)).send().await {
            Ok(it) => it,
            Err(err) => {
                println!("Error when sending auth request: {}", err);
                return OAuthCallback::AuthenticationError;
            }
        };

        if !res.status().is_success() {
            println!("{}", res.text().await.unwrap());
            return OAuthCallback::AuthenticationError;
        }

        let text = match res.text().await {
            Ok(it) => it,
            Err(err) => {
                println!("Error when getting body: {}", err);
                return OAuthCallback::AuthenticationError;
            }
        };

        let data: OauthResponse =
            from_str(&text).expect("Google should provide valid JSON all the time");

        login_user(data, &self.pool);


        OAuthCallback::SuccessfullyAuthenticated("/".to_string())
    }
}

#[derive(Deserialize)]
pub struct OauthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_in: String,
    pub scope: String,
    pub token_type: String,
    #[serde(rename = "id_token")]
    pub jwt: String,
}

#[derive(Serialize)]
struct OAuthCallbackOptions {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Serialize)]
struct OAuthRequestOptions {
    redirect_uri: String,
    client_id: String,
    access_type: String,
    response_type: String,
    prompt: String,
    scope: String,
}

#[derive(ApiResponse)]
enum OAuthCallback {
    /// When everything goes right and the user successfully authenticates themselves
    #[oai(status = "200")]
    SuccessfullyAuthenticated(#[oai(header = "Location")] String),
    /// When something went wrong during authentication on the server side
    #[oai(status = "500")]
    AuthenticationError,
}

#[derive(ApiResponse)]
enum GoogleAuthRedirect {
    #[oai(status = "302")]
    SuccessfulRedirect(#[oai(header = "Location")] String),
}
