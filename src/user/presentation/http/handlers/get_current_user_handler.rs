use crate::presentation::http::auth::OidcIdentity;
use crate::shared::domain::UserId;

/// Builds the current-user response from the verified identity claims.
pub fn handle(
    user_id: &UserId,
    identity: &OidcIdentity,
) -> dash_api::models::GetCurrentUser200Response {
    let mut response = dash_api::models::GetCurrentUser200Response::new(
        user_id.as_str().to_owned(),
        identity.username.clone(),
        identity.groups.clone(),
    );
    response.first_name = identity
        .first_name
        .clone()
        .map(dash_api::types::Nullable::Present);
    response.last_name = identity
        .last_name
        .clone()
        .map(dash_api::types::Nullable::Present);
    response.profile_picture = identity
        .profile_picture
        .clone()
        .map(dash_api::types::Nullable::Present);
    response
}
