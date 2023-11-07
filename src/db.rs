use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use serde::{Serialize, Deserialize, de};
use sqlx::{Pool, Postgres, query};

use crate::api::auth::OauthResponse;

pub async fn login_user(response: OauthResponse, pool: &Pool<Postgres>) {
    // response.
    // let decoded_token = decode::<Claims, DecodingKey<Algorithm>>(&response.jwt, DecodingKey::, &Validation::default()).unwrap();
    // let claims = decoded_token.claims;

    query!("SELECT COUNT(1) FROM auth WHERE google_sub = $1", "TODO")
        .fetch_one(pool)
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}