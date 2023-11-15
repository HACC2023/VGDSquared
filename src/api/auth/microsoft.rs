use const_format::concatcp;
use serde::Serialize;
use dotenv_codegen::dotenv;

use super::{AuthApi, OAuthCallbackResponse};

impl AuthApi {
    const MICROSOFT: MicrosoftOauthConst = MicrosoftOauthConst {
        auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        token_url: concatcp!(
            "https://login.microsoftonline.com/",
            dotenv!("MICROSOFT_CLIENT_TENANT"),
            "/oauth2/v2.0/token",
        ),
        id: dotenv!("MICROSOFT_CLIENT_ID"),
        secret: dotenv!("MICROSOFT_CLIENT_SECRET"),
        tenant: dotenv!("MICROSOFT_CLIENT_TENANT"),
        callback_url: "http://localhost:3000/api/oauth/microsoft/callback",
    };

    pub(super) fn microsoft_redirect_string() -> String {
        type Api = AuthApi;

        #[derive(Serialize)]
        struct MicrosoftRequestOptions {
            redirect_uri: String,
            client_id: String,
            access_type: String,
            response_type: String,
            scope: String,
        }

        let options = MicrosoftRequestOptions {
            redirect_uri: Api::MICROSOFT.callback_url.to_string(),
            client_id: Api::MICROSOFT.id.to_string(),
            access_type: "offline".to_string(),
            response_type: "code".to_string(),
            scope: "https://graph.microsoft.com/user.read".to_string(),
        };

        let serialized = serde_qs::to_string(&options).expect("It decided to fail");
        format!("{}?{serialized}", Api::MICROSOFT.auth_url)
    }
    pub async fn ms_callback(&self, code: String) -> OAuthCallbackResponse {
        #[derive(Serialize)]
        struct MicrosoftCallbackMessage {
            code: String,
            client_id: String,
            client_secret: String,
            redirect_uri: String,
            grant_type: String,
            scope: String,
        }

        let to_send = MicrosoftCallbackMessage {
            code: code,
            client_id: Self::MICROSOFT.id.to_string(),
            client_secret: Self::MICROSOFT.secret.to_string(),
            redirect_uri: Self::MICROSOFT.callback_url.to_string(),
            grant_type: "authorization_code".to_string(),
            scope: "https://graph.microsoft.com/.default".to_string(),
        };

        let serialized = serde_qs::to_string(&to_send).unwrap();
        self.request_google_user(Self::MICROSOFT.token_url, serialized)
            .await
    }
}

struct MicrosoftOauthConst {
    auth_url: &'static str,
    token_url: &'static str,
    id: &'static str,
    secret: &'static str,
    callback_url: &'static str,
    tenant: &'static str,
}