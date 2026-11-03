use crate::{
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::{self, ApiJson, HttpError},
        middleware::require_scope,
    },
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId, ThemeMode},
};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

pub fn router() -> Router<HttpApi> {
    Router::new()
        .route("/themes", get(list_themes).post(create_theme))
        .route("/themes/{theme_id}", get(get_theme).delete(delete_theme))
}
fn model(theme: Theme) -> dash_api::models::CreateTheme201Response {
    dash_api::models::CreateTheme201Response::new(
        String::from(theme.id()),
        theme.name().to_owned(),
        match theme.mode() {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
        .to_owned(),
        dash_api::models::ListThemes200ResponseInnerPalette::new(
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
}
fn parse(raw: &str) -> Result<ThemeId, HttpError> {
    ThemeId::try_from(raw).map_err(|_| HttpError(ApplicationError::BadRequest))
}
async fn list_themes(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_themes_read")?;
    let themes = crate::theme::presentation::http::handlers::list_themes_handler::handle(
        api.themes,
        &claims.user_id,
    )
    .await?;
    let result: Vec<_> = themes.into_iter().map(model).collect();
    Ok(error::json(StatusCode::OK, result))
}
async fn get_theme(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<String>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_themes_read")?;
    let theme = crate::theme::presentation::http::handlers::get_theme_handler::handle(
        api.themes,
        &claims.user_id,
        parse(&id)?,
    )
    .await?;
    Ok(error::json(StatusCode::OK, model(theme)))
}
async fn create_theme(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::CreateThemeRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_themes_write")?;
    let id = crate::theme::presentation::http::handlers::create_theme_handler::handle(
        api.themes.clone(),
        claims.user_id.clone(),
        &body,
    )
    .await?;
    let theme = crate::theme::presentation::http::handlers::get_theme_handler::handle(
        api.themes,
        &claims.user_id,
        id,
    )
    .await?;
    Ok(error::json(StatusCode::CREATED, model(theme)))
}
async fn delete_theme(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<String>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_themes_write")?;
    crate::theme::presentation::http::handlers::delete_theme_handler::handle(
        api.preferences,
        api.themes,
        &claims.user_id,
        parse(&id)?,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
