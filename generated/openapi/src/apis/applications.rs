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
pub enum CreateApplicationResponse {
    /// The shared app has been added.
    Status201_TheSharedAppHasBeenAdded(models::CreateApplication201Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteApplicationResponse {
    /// The app has been removed from the shared catalog.
    Status204_TheAppHasBeenRemovedFromTheSharedCatalog,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetApplicationResponse {
    /// The shared app's name, icon, address, and visibility.
    Status200_TheSharedApp(models::CreateApplication201Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListApplicationsResponse {
    /// All apps in the shared catalog, sorted by the selected rule and including their visibility settings.
    Status200_AllAppsInTheSharedCatalog(models::ListApplications200Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RecordApplicationAccessResponse {
    /// The app has been added to your recent-use list.
    Status204_TheAppHasBeenAddedToYourRecent,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateApplicationResponse {
    /// The shared app has been updated.
    Status200_TheSharedAppHasBeenUpdated(models::CreateApplication201Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateApplicationCatalogOrderResponse {
    /// Your custom app order has been saved.
    Status204_YourCustomAppOrderHasBeenSaved,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateApplicationCatalogSortOrderResponse {
    /// The app catalog sorting rule has been saved.
    Status204_TheAppCatalogSortingRuleHasBeenSaved,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// You don't have permission to make this change.
    Status403_YouDon(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

/// Applications
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Applications<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    type Claims;

    /// Add an app to the shared catalog.
    ///
    /// CreateApplication - POST /api/v1/applications
    async fn create_application(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, E>;

    /// Delete a shared app.
    ///
    /// DeleteApplication - DELETE /api/v1/applications/{application_id}
    async fn delete_application(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::DeleteApplicationPathParams,
    ) -> Result<DeleteApplicationResponse, E>;

    /// View a shared app's settings.
    ///
    /// GetApplication - GET /api/v1/applications/{application_id}
    async fn get_application(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::GetApplicationPathParams,
    ) -> Result<GetApplicationResponse, E>;

    /// Browse the shared app catalog.
    ///
    /// ListApplications - GET /api/v1/applications
    async fn list_applications(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<ListApplicationsResponse, E>;

    /// Mark an app as opened.
    ///
    /// RecordApplicationAccess - POST /api/v1/applications/{application_id}/access
    async fn record_application_access(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::RecordApplicationAccessPathParams,
    ) -> Result<RecordApplicationAccessResponse, E>;

    /// Update a shared app.
    ///
    /// UpdateApplication - PUT /api/v1/applications/{application_id}
    async fn update_application(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::UpdateApplicationPathParams,
        body: &models::UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, E>;

    /// Change the order of the apps.
    ///
    /// UpdateApplicationCatalogOrder - PUT /api/v1/applications/order
    async fn update_application_catalog_order(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::UpdateApplicationCatalogOrderRequest,
    ) -> Result<UpdateApplicationCatalogOrderResponse, E>;

    /// Update the app catalog sorting rule.
    ///
    /// UpdateApplicationCatalogSortOrder - PUT /api/v1/applications
    async fn update_application_catalog_sort_order(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::UpdateCategoriesSortOrderRequest,
    ) -> Result<UpdateApplicationCatalogSortOrderResponse, E>;
}
