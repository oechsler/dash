use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use dash_api::{apis::themes as api, models};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    shared::application::{ApplicationError, ApplicationResult},
    theme::domain::{Theme, ThemeId, ThemeMode},
};

fn scope(c: &HttpClaims, value: &str) -> ApplicationResult<()> {
    if c.allows(value) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}
fn model(theme: Theme) -> models::CreateTheme201Response {
    let id = String::from(theme.id());
    let scope = if theme.is_deletable() {
        "user"
    } else {
        "global"
    }
    .to_owned();
    models::CreateTheme201Response::new(
        id,
        theme.name().to_owned(),
        match theme.mode() {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
        .to_owned(),
        models::ListThemes200ResponseInnerPalette::new(
            theme.palette().primary().as_str().to_owned(),
            theme.palette().secondary().as_str().to_owned(),
            theme.palette().tertiary().as_str().to_owned(),
        ),
        scope,
    )
}
fn parse_id(raw: &str) -> ApplicationResult<ThemeId> {
    ThemeId::try_from(raw).map_err(ApplicationError::from)
}

#[async_trait]
impl api::Themes<ApplicationError> for HttpApi {
    type Claims = HttpClaims;
    async fn create_theme(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        body: &models::CreateThemeRequest,
    ) -> ApplicationResult<api::CreateThemeResponse> {
        scope(c, "dash_themes_write")?;
        let id = crate::theme::presentation::http::handlers::create_theme_handler::handle(
            self.themes.clone(),
            c.user_id.clone(),
            body,
        )
        .await?;
        let theme = crate::theme::presentation::http::handlers::get_theme_handler::handle(
            self.themes.clone(),
            &c.user_id,
            id,
        )
        .await?;
        Ok(api::CreateThemeResponse::Status201_YourNewThemeIsReadyToUse(model(theme)))
    }
    async fn delete_theme(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::DeleteThemePathParams,
    ) -> ApplicationResult<api::DeleteThemeResponse> {
        scope(c, "dash_themes_write")?;
        crate::theme::presentation::http::handlers::delete_theme_handler::handle(
            self.preferences.clone(),
            self.themes.clone(),
            &c.user_id,
            parse_id(&p.theme_id)?,
        )
        .await?;
        Ok(api::DeleteThemeResponse::Status204_YourThemeHasBeenDeleted)
    }
    async fn get_theme(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::GetThemePathParams,
    ) -> ApplicationResult<api::GetThemeResponse> {
        scope(c, "dash_themes_read")?;
        let theme = crate::theme::presentation::http::handlers::get_theme_handler::handle(
            self.themes.clone(),
            &c.user_id,
            parse_id(&p.theme_id)?,
        )
        .await?;
        Ok(api::GetThemeResponse::Status200_TheTheme(model(theme)))
    }
    async fn list_themes(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
    ) -> ApplicationResult<api::ListThemesResponse> {
        scope(c, "dash_themes_read")?;
        let themes = crate::theme::presentation::http::handlers::list_themes_handler::handle(
            self.themes.clone(),
            &c.user_id,
        )
        .await?;
        let output = themes
            .into_iter()
            .map(|theme| {
                models::ListThemes200ResponseInner::new(
                    String::from(theme.id()),
                    theme.name().to_owned(),
                    match theme.mode() {
                        ThemeMode::Light => "light",
                        ThemeMode::Dark => "dark",
                    }
                    .to_owned(),
                    models::ListThemes200ResponseInnerPalette::new(
                        theme.palette().primary().as_str().to_owned(),
                        theme.palette().secondary().as_str().to_owned(),
                        theme.palette().tertiary().as_str().to_owned(),
                    ),
                    if theme.is_deletable() {
                        "user"
                    } else {
                        "global"
                    }
                    .to_owned(),
                )
            })
            .collect();
        Ok(api::ListThemesResponse::Status200_Built(output))
    }
}
