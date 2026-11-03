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
pub enum GetPreferencesResponse {
    /// Your language, time zone, and selected themes.
    Status200_YourLanguage(models::GetPreferences200Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePreferencesResponse {
    /// Your preferences have been saved.
    Status200_YourPreferencesHaveBeenSaved(models::GetPreferences200Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

/// Preferences
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Preferences<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    type Claims;

    /// View your preferences.
    ///
    /// GetPreferences - GET /api/v1/preferences
    async fn get_preferences(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<GetPreferencesResponse, E>;

    /// Update your preferences.
    ///
    /// UpdatePreferences - PUT /api/v1/preferences
    async fn update_preferences(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::UpdatePreferencesRequest,
    ) -> Result<UpdatePreferencesResponse, E>;
}
