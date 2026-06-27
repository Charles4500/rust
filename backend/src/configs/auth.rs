use oauth2::{
    basic::*, AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl,
    StandardRevocableToken, TokenUrl,
};
use std::env;

type GoogleOAuthClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;
pub fn get_google_oauth_client(
    client_id: Option<String>,
    client_secret: Option<String>,
    redirect_url: Option<String>,
) -> GoogleOAuthClient {
    let client_id = client_id
        .unwrap_or_else(|| env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));
    let client_secret = client_secret.unwrap_or_else(|| {
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set")
    });
    let redirect_url = redirect_url.unwrap_or_else(|| {
        env::var("REDIRECT_URL")
            .unwrap_or_else(|_| "http://localhost:3000/auth/google/callback".to_string())
    });

    BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        )
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
