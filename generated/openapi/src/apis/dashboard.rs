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
pub enum CreateBookmarkResponse {
    /// Your link has been saved.
    Status201_YourLinkHasBeenSaved(models::CreateBookmark201Response),
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateCategoryResponse {
    /// Your new category is ready to use.
    Status201_YourNewCategoryIsReadyToUse(models::CreateCategory201Response),
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteBookmarkResponse {
    /// Your link has been removed.
    Status204_YourLinkHasBeenRemoved,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteCategoryResponse {
    /// The category and its bookmarks have been deleted.
    Status204_TheCategoryAndItsBookmarksHaveBeenDeleted,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetDashboardResponse {
    /// Your apps, active categories, and shelved categories, sorted by your selected category sorting rule.
    Status200_YourApps(models::GetDashboard200Response),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RecordBookmarkAccessResponse {
    /// Your bookmark has been marked as opened.
    Status204_YourBookmarkHasBeenMarkedAsOpened,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReorderBookmarksResponse {
    /// The link order has been saved.
    Status204_TheLinkOrderHasBeenSaved,
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ReorderCategoriesResponse {
    /// Your category order has been saved.
    Status204_YourCategoryOrderHasBeenSaved,
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateBookmarkResponse {
    /// Your link has been updated.
    Status200_YourLinkHasBeenUpdated(models::CreateBookmark201Response),
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateCategoriesSortOrderResponse {
    /// Your category sorting rule has been saved.
    Status204_YourCategorySortingRuleHasBeenSaved,
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateCategoryResponse {
    /// Your category has been updated.
    Status200_YourCategoryHasBeenUpdated(models::CreateCategory201Response),
    /// We couldn't understand the request. Check the values and try again.
    Status400_WeCouldn(models::Problem),
    /// Sign in to continue.
    Status401_SignInToContinue(models::Problem),
    /// We couldn't find that item.
    Status404_WeCouldn(models::Problem),
    /// One or more values need to be corrected.
    Status422_OneOrMoreValuesNeedToBeCorrected(models::Problem),
}

/// Dashboard
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Dashboard<E: std::fmt::Debug + Send + Sync + 'static = ()>:
    super::ErrorHandler<E>
{
    type Claims;

    /// Add a link to a category.
    ///
    /// CreateBookmark - POST /api/v1/categories/{category_id}/bookmarks
    async fn create_bookmark(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::CreateBookmarkPathParams,
        body: &models::CreateBookmarkRequest,
    ) -> Result<CreateBookmarkResponse, E>;

    /// Create a category for your links.
    ///
    /// CreateCategory - POST /api/v1/categories
    async fn create_category(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::CreateCategoryRequest,
    ) -> Result<CreateCategoryResponse, E>;

    /// Remove a saved link.
    ///
    /// DeleteBookmark - DELETE /api/v1/categories/{category_id}/bookmarks/{bookmark_id}
    async fn delete_bookmark(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::DeleteBookmarkPathParams,
    ) -> Result<DeleteBookmarkResponse, E>;

    /// Delete a category and all its links.
    ///
    /// DeleteCategory - DELETE /api/v1/categories/{category_id}
    async fn delete_category(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::DeleteCategoryPathParams,
    ) -> Result<DeleteCategoryResponse, E>;

    /// View your dashboard.
    ///
    /// GetDashboard - GET /api/v1/dashboard
    async fn get_dashboard(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
    ) -> Result<GetDashboardResponse, E>;

    /// Mark a bookmark as opened.
    ///
    /// RecordBookmarkAccess - POST /api/v1/categories/{category_id}/bookmarks/{bookmark_id}/access
    async fn record_bookmark_access(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::RecordBookmarkAccessPathParams,
    ) -> Result<RecordBookmarkAccessResponse, E>;

    /// Change the order of links in a category.
    ///
    /// ReorderBookmarks - PUT /api/v1/categories/{category_id}/bookmarks/order
    async fn reorder_bookmarks(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::ReorderBookmarksPathParams,
        body: &models::ReorderBookmarksRequest,
    ) -> Result<ReorderBookmarksResponse, E>;

    /// Change the order of your categories.
    ///
    /// ReorderCategories - PUT /api/v1/categories/order
    async fn reorder_categories(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::ReorderCategoriesRequest,
    ) -> Result<ReorderCategoriesResponse, E>;

    /// Update or move a saved link.
    ///
    /// UpdateBookmark - PUT /api/v1/categories/{category_id}/bookmarks/{bookmark_id}
    async fn update_bookmark(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::UpdateBookmarkPathParams,
        body: &models::CreateBookmarkRequest,
    ) -> Result<UpdateBookmarkResponse, E>;

    /// Update the category sorting rule.
    ///
    /// UpdateCategoriesSortOrder - PUT /api/v1/categories
    async fn update_categories_sort_order(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        body: &models::UpdateCategoriesSortOrderRequest,
    ) -> Result<UpdateCategoriesSortOrderResponse, E>;

    /// Update a category.
    ///
    /// UpdateCategory - PUT /api/v1/categories/{category_id}
    async fn update_category(
        &self,

        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::UpdateCategoryPathParams,
        body: &models::UpdateCategoryRequest,
    ) -> Result<UpdateCategoryResponse, E>;
}
