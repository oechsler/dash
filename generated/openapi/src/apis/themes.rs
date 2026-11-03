use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateThemeResponse {
    /// Your new theme is ready to use.
    Status201_YourNewThemeIsReadyToUse(models::CreateTheme201Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteThemeResponse {
    /// Your theme has been deleted.
    Status204_YourThemeHasBeenDeleted,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// You can't delete a built-in theme or one you're using.
    Status409_YouCan(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetThemeResponse {
    /// The theme's name, appearance, and colors.
    Status200_TheTheme(models::CreateTheme201Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListThemesResponse {
    /// Built-in themes and themes you've created.
    Status200_Built(Vec<models::ListThemes200ResponseInner>),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

/// Themes
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Themes<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    type Claims;

    /// Create your own theme.
    ///
    /// CreateTheme - POST /api/v1/themes
    async fn create_theme(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::CreateThemeRequest,
    ) -> Result<CreateThemeResponse, E>;

    /// Delete one of your themes.
    ///
    /// DeleteTheme - DELETE /api/v1/themes/{theme_id}
    async fn delete_theme(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::DeleteThemePathParams,
    ) -> Result<DeleteThemeResponse, E>;

    /// View a theme.
    ///
    /// GetTheme - GET /api/v1/themes/{theme_id}
    async fn get_theme(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::GetThemePathParams,
    ) -> Result<GetThemeResponse, E>;

    /// Browse available themes.
    ///
    /// ListThemes - GET /api/v1/themes
    async fn list_themes(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<ListThemesResponse, E>;
}
