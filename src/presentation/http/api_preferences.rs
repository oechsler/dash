use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use dash_api::{apis::preferences as api, models, types::Nullable};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    preferences::domain::UserPreferences,
    shared::application::{ApplicationError, ApplicationResult},
    theme::domain::ThemeMode,
};

fn selected_id(preferences: &UserPreferences, mode: ThemeMode) -> Nullable<String> {
    match preferences.theme_id(mode) {
        Some(id) => Nullable::Present(String::from(id)),
        None => Nullable::Null,
    }
}

fn response(preferences: UserPreferences) -> models::GetPreferences200Response {
    models::GetPreferences200Response::new(
        preferences.language().as_str().to_owned(),
        preferences.time_zone().as_str().to_owned(),
        models::GetPreferences200ResponseSelectedThemes::new(
            selected_id(&preferences, ThemeMode::Light),
            selected_id(&preferences, ThemeMode::Dark),
        ),
    )
}

fn require_scope(claims: &HttpClaims, name: &str) -> ApplicationResult<()> {
    if claims.allows(name) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}

#[async_trait]
impl api::Preferences<ApplicationError> for HttpApi {
    type Claims = HttpClaims;

    async fn get_preferences(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
    ) -> ApplicationResult<api::GetPreferencesResponse> {
        require_scope(claims, "dash_preferences_read")?;
        let preferences =
            crate::preferences::presentation::http::handlers::get_preferences_handler::handle(
                self.preferences.clone(),
                &claims.user_id,
            )
            .await?;
        Ok(api::GetPreferencesResponse::Status200_YourLanguage(
            response(preferences),
        ))
    }

    async fn update_preferences(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        body: &models::UpdatePreferencesRequest,
    ) -> ApplicationResult<api::UpdatePreferencesResponse> {
        require_scope(claims, "dash_preferences_write")?;
        crate::preferences::presentation::http::handlers::update_preferences_handler::handle(
            self.preferences.clone(),
            self.themes.clone(),
            claims.user_id.clone(),
            body,
        )
        .await?;
        let preferences =
            crate::preferences::presentation::http::handlers::get_preferences_handler::handle(
                self.preferences.clone(),
                &claims.user_id,
            )
            .await?;
        Ok(
            api::UpdatePreferencesResponse::Status200_YourPreferencesHaveBeenSaved(response(
                preferences,
            )),
        )
    }
}
