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
pub enum BeginLoginResponse {
    /// Continue to your account provider to sign in.
    Status302_ContinueToYourAccountProviderToSignIn { location: Option<String> },
    /// Something went wrong while handling the request.
    Status500_SomethingWentWrongWhileHandlingTheRequest(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CompleteLoginResponse {
    /// Redirects back to the app after sign-in or with a readable error if sign-in failed.
    Status302_RedirectsBackToTheAppAfterSign {
        location: Option<String>,
        set_cookie: Option<String>,
    },
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DowngradeSessionResponse {
    /// This device will use a short-lived sign-in.
    Status204_ThisDeviceWillUseAShort,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// This device is already using a short-lived sign-in.
    Status409_ThisDeviceIsAlreadyUsingAShort(models::UpgradeSession409Response),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum InvalidateSessionResponse {
    /// That device has been signed out.
    Status204_ThatDeviceHasBeenSignedOut,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListSessionsResponse {
    /// Your signed-in devices, including this one.
    Status200_YourSigned(Vec<models::ListSessions200ResponseInner>),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LogoutResponse {
    /// Redirect to the account provider to complete sign-out.
    Status303_RedirectToTheAccountProviderToCompleteSign {
        location: Option<String>,
        set_cookie: Option<String>,
    },
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TerminateSessionResponse {
    /// That device has been signed out.
    Status204_ThatDeviceHasBeenSignedOut,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpgradeSessionResponse {
    /// This device will stay signed in longer.
    Status204_ThisDeviceWillStaySignedInLonger,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// This device is already using a long-lived sign-in.
    Status409_ThisDeviceIsAlreadyUsingALong(models::UpgradeSession409Response),
}

/// Sessions
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Sessions<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    type Claims;

    /// Sign in.
    ///
    /// BeginLogin - GET /api/v1/sessions/login
    async fn begin_login(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<BeginLoginResponse, E>;

    /// Finish signing in.
    ///
    /// CompleteLogin - GET /api/v1/sessions/login/callback
    async fn complete_login(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::CompleteLoginQueryParams,
    ) -> Result<CompleteLoginResponse, E>;

    /// Use a shorter sign-in on this device.
    ///
    /// DowngradeSession - POST /api/v1/sessions/{session_id}/downgrade
    async fn downgrade_session(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::DowngradeSessionPathParams,
    ) -> Result<DowngradeSessionResponse, E>;

    /// Sign out a device.
    ///
    /// InvalidateSession - DELETE /api/v1/sessions/{session_id}
    async fn invalidate_session(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::InvalidateSessionPathParams,
    ) -> Result<InvalidateSessionResponse, E>;

    /// See where you're signed in.
    ///
    /// ListSessions - GET /api/v1/sessions
    async fn list_sessions(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<ListSessionsResponse, E>;

    /// Sign out.
    ///
    /// Logout - POST /api/v1/sessions/logout
    async fn logout(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<LogoutResponse, E>;

    /// End a device session.
    ///
    /// TerminateSession - POST /api/v1/sessions/{session_id}/terminate
    async fn terminate_session(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::TerminateSessionPathParams,
    ) -> Result<TerminateSessionResponse, E>;

    /// Keep this device signed in longer.
    ///
    /// UpgradeSession - POST /api/v1/sessions/{session_id}/upgrade
    async fn upgrade_session(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::UpgradeSessionPathParams,
    ) -> Result<UpgradeSessionResponse, E>;
}
