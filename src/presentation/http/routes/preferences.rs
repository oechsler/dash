use crate::{
    preferences::domain::UserPreferences,
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::{self, ApiJson, HttpError},
        middleware::require_scope,
    },
};
use axum::{
    Router, extract::State, http::StatusCode, response::Response, routing::get as get_route,
};

pub fn router() -> Router<HttpApi> {
    Router::new().route(
        "/preferences",
        get_route(get_preferences).put(update_preferences),
    )
}

fn response(preferences: UserPreferences) -> dash_api::models::GetPreferences200Response {
    use dash_api::types::Nullable;
    let selected = |mode| match preferences.theme_id(mode) {
        Some(id) => Nullable::Present(String::from(id)),
        None => Nullable::Null,
    };
    dash_api::models::GetPreferences200Response::new(
        preferences.language().as_str().to_owned(),
        preferences.time_zone().as_str().to_owned(),
        dash_api::models::GetPreferences200ResponseSelectedThemes::new(
            selected(crate::theme::domain::ThemeMode::Light),
            selected(crate::theme::domain::ThemeMode::Dark),
        ),
    )
}

async fn get_preferences(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_preferences_read")?;
    let preferences =
        crate::preferences::presentation::http::handlers::get_preferences_handler::handle(
            api.preferences,
            &claims.user_id,
        )
        .await?;
    Ok(error::json(StatusCode::OK, response(preferences)))
}

async fn update_preferences(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::UpdatePreferencesRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_preferences_write")?;
    crate::preferences::presentation::http::handlers::update_preferences_handler::handle(
        api.preferences.clone(),
        api.themes,
        claims.user_id.clone(),
        &body,
    )
    .await?;
    let preferences =
        crate::preferences::presentation::http::handlers::get_preferences_handler::handle(
            api.preferences,
            &claims.user_id,
        )
        .await?;
    Ok(error::json(StatusCode::OK, response(preferences)))
}
