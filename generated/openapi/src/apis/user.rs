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
pub enum DeleteCurrentUserResponse {
    /// Your data has been deleted.
    Status204_YourDataHasBeenDeleted,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCurrentUserResponse {
    /// Your profile and the groups that give you access to shared apps.
    Status200_YourProfileAndTheGroupsThatGiveYouAccessToSharedApps(
        models::GetCurrentUser200Response,
    ),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

/// User
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait User<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    type Claims;

    /// Delete your account.
    ///
    /// DeleteCurrentUser - DELETE /api/v1/me
    async fn delete_current_user(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<DeleteCurrentUserResponse, E>;

    /// View your account profile.
    ///
    /// GetCurrentUser - GET /api/v1/me
    async fn get_current_user(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<GetCurrentUserResponse, E>;
}
