use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use oauth2::{basic::*, EndpointNotSet, EndpointSet, StandardRevocableToken};

pub type GoogleOAuthClient = oauth2::Client<
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

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<AsyncPgConnection>,
    pub jwt_secret: String,
    pub google_client: GoogleOAuthClient,
}
