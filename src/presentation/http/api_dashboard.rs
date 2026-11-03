use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use dash_api::{apis::dashboard as api, models};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    dashboard::{
        application::queries::DashboardView,
        domain::{Bookmark, Category},
    },
    shared::application::{ApplicationError, ApplicationResult},
};

fn check(claims: &HttpClaims, scope: &str) -> ApplicationResult<()> {
    if claims.allows(scope) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}
fn bookmarks(
    items: &[Bookmark],
) -> Vec<models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner> {
    items
        .iter()
        .map(|item| {
            models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner::new(
                item.id().as_uuid(),
                item.name().to_owned(),
                item.icon().as_str().to_owned(),
                item.address().as_str().to_owned(),
            )
        })
        .collect()
}
fn category(item: &Category) -> models::GetDashboard200ResponseShelvedCategoriesInner {
    models::GetDashboard200ResponseShelvedCategoriesInner::new(
        item.id().as_uuid(),
        item.name().to_owned(),
        item.bookmarks_sort_order().as_str().to_owned(),
        bookmarks(item.bookmarks()),
    )
}
fn dashboard(view: DashboardView) -> models::GetDashboard200Response {
    let applications = view
        .applications
        .into_iter()
        .map(|app| {
            let mut item = models::GetDashboard200ResponseApplicationsInner::new(
                app.id().as_uuid(),
                app.name().to_owned(),
                app.icon().as_str().to_owned(),
                app.address().as_str().to_owned(),
            );
            item.health = app.health().map(|health| match health {
                crate::catalog::domain::ApplicationHealth::Up => "up".to_owned(),
                crate::catalog::domain::ApplicationHealth::Down => "down".to_owned(),
            });
            item
        })
        .collect();
    models::GetDashboard200Response::new(
        applications,
        view.categories_sort_order.as_str().to_owned(),
        view.shelved_categories
            .iter()
            .map(|c| category(c))
            .collect(),
        view.categories.iter().map(category).collect(),
    )
}

#[async_trait]
impl api::Dashboard<ApplicationError> for HttpApi {
    type Claims = HttpClaims;

    async fn create_bookmark(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::CreateBookmarkPathParams,
        b: &models::CreateBookmarkRequest,
    ) -> ApplicationResult<api::CreateBookmarkResponse> {
        check(c, "dash_dashboard_write")?;
        let id = crate::dashboard::presentation::http::handlers::create_bookmark_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            p.category_id.into(),
            b,
        )
        .await?;
        let item =
            crate::dashboard::application::queries::GetUserBookmark::new(self.dashboard.clone())
                .execute(&c.user_id, id)
                .await?;
        Ok(api::CreateBookmarkResponse::Status201_YourLinkHasBeenSaved(
            models::CreateBookmark201Response::new(
                item.id().as_uuid(),
                item.name().to_owned(),
                item.icon().as_str().to_owned(),
                item.address().as_str().to_owned(),
            ),
        ))
    }
    async fn create_category(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        b: &models::CreateCategoryRequest,
    ) -> ApplicationResult<api::CreateCategoryResponse> {
        check(c, "dash_dashboard_write")?;
        let id = crate::dashboard::presentation::http::handlers::create_category_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            b,
        )
        .await?;
        let item =
            crate::dashboard::application::queries::GetUserCategory::new(self.dashboard.clone())
                .execute(&c.user_id, id)
                .await?;
        Ok(
            api::CreateCategoryResponse::Status201_YourNewCategoryIsReadyToUse(
                models::CreateCategory201Response::new(
                    item.id().as_uuid(),
                    item.name().to_owned(),
                    item.bookmarks_sort_order().as_str().to_owned(),
                    bookmarks(item.bookmarks()),
                ),
            ),
        )
    }
    async fn delete_bookmark(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::DeleteBookmarkPathParams,
    ) -> ApplicationResult<api::DeleteBookmarkResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::delete_bookmark_handler::handle(
            self.dashboard.clone(),
            &c.user_id,
            p.category_id.into(),
            p.bookmark_id.into(),
        )
        .await?;
        Ok(api::DeleteBookmarkResponse::Status204_YourLinkHasBeenRemoved)
    }
    async fn delete_category(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::DeleteCategoryPathParams,
    ) -> ApplicationResult<api::DeleteCategoryResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::delete_category_handler::handle(
            self.dashboard.clone(),
            &c.user_id,
            p.category_id.into(),
        )
        .await?;
        Ok(api::DeleteCategoryResponse::Status204_TheCategoryAndItsBookmarksHaveBeenDeleted)
    }
    async fn get_dashboard(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
    ) -> ApplicationResult<api::GetDashboardResponse> {
        check(c, "dash_dashboard_read")?;
        let view = crate::dashboard::presentation::http::handlers::get_dashboard_handler::handle(
            self.dashboard.clone(),
            self.catalog.clone(),
            self.catalog.clone(),
            self.preferences.clone(),
            &c.user_id,
            &c.groups,
        )
        .await?;
        Ok(api::GetDashboardResponse::Status200_YourApps(dashboard(
            view,
        )))
    }
    async fn record_bookmark_access(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::RecordBookmarkAccessPathParams,
    ) -> ApplicationResult<api::RecordBookmarkAccessResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::record_bookmark_access_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            p.category_id.into(),
            p.bookmark_id.into(),
        )
        .await?;
        Ok(api::RecordBookmarkAccessResponse::Status204_YourBookmarkHasBeenMarkedAsOpened)
    }
    async fn reorder_bookmarks(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::ReorderBookmarksPathParams,
        b: &models::ReorderBookmarksRequest,
    ) -> ApplicationResult<api::ReorderBookmarksResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::reorder_bookmarks_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            p.category_id.into(),
            b,
        )
        .await?;
        Ok(api::ReorderBookmarksResponse::Status204_TheLinkOrderHasBeenSaved)
    }
    async fn reorder_categories(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        b: &models::ReorderCategoriesRequest,
    ) -> ApplicationResult<api::ReorderCategoriesResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::reorder_categories_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            b,
        )
        .await?;
        Ok(api::ReorderCategoriesResponse::Status204_YourCategoryOrderHasBeenSaved)
    }
    async fn update_bookmark(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::UpdateBookmarkPathParams,
        b: &models::CreateBookmarkRequest,
    ) -> ApplicationResult<api::UpdateBookmarkResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::update_bookmark_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            p.category_id.into(),
            p.bookmark_id.into(),
            b,
        )
        .await?;
        let item =
            crate::dashboard::application::queries::GetUserBookmark::new(self.dashboard.clone())
                .execute(&c.user_id, p.bookmark_id.into())
                .await?;
        Ok(
            api::UpdateBookmarkResponse::Status200_YourLinkHasBeenUpdated(
                models::CreateBookmark201Response::new(
                    item.id().as_uuid(),
                    item.name().to_owned(),
                    item.icon().as_str().to_owned(),
                    item.address().as_str().to_owned(),
                ),
            ),
        )
    }
    async fn update_categories_sort_order(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        b: &models::UpdateCategoriesSortOrderRequest,
    ) -> ApplicationResult<api::UpdateCategoriesSortOrderResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::update_categories_sort_order_handler::handle(self.dashboard.clone(),c.user_id.clone(),b).await?;
        Ok(api::UpdateCategoriesSortOrderResponse::Status204_YourCategorySortingRuleHasBeenSaved)
    }
    async fn update_category(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::UpdateCategoryPathParams,
        b: &models::UpdateCategoryRequest,
    ) -> ApplicationResult<api::UpdateCategoryResponse> {
        check(c, "dash_dashboard_write")?;
        crate::dashboard::presentation::http::handlers::update_category_handler::handle(
            self.dashboard.clone(),
            c.user_id.clone(),
            p.category_id.into(),
            b,
        )
        .await?;
        let item =
            crate::dashboard::application::queries::GetUserCategory::new(self.dashboard.clone())
                .execute(&c.user_id, p.category_id.into())
                .await?;
        Ok(
            api::UpdateCategoryResponse::Status200_YourCategoryHasBeenUpdated(
                models::CreateCategory201Response::new(
                    item.id().as_uuid(),
                    item.name().to_owned(),
                    item.bookmarks_sort_order().as_str().to_owned(),
                    bookmarks(item.bookmarks()),
                ),
            ),
        )
    }
}
