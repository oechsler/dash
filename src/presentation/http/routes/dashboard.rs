use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
};
use uuid::Uuid;

use crate::{
    dashboard::{
        application::queries::DashboardView,
        domain::{Bookmark, Category},
    },
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::{self, ApiJson, HttpError},
        middleware::require_scope,
    },
};

pub fn router() -> Router<HttpApi> {
    Router::new()
        .route("/dashboard", get(get_dashboard))
        .route(
            "/categories",
            post(create_category).put(update_categories_sort_order),
        )
        .route("/categories/order", put(reorder_categories))
        .route(
            "/categories/{category_id}",
            put(update_category).delete(delete_category),
        )
        .route("/categories/{category_id}/bookmarks", post(create_bookmark))
        .route(
            "/categories/{category_id}/bookmarks/order",
            put(reorder_bookmarks),
        )
        .route(
            "/categories/{category_id}/bookmarks/{bookmark_id}",
            put(update_bookmark).delete(delete_bookmark),
        )
        .route(
            "/categories/{category_id}/bookmarks/{bookmark_id}/access",
            post(record_bookmark_access),
        )
}

fn bookmarks(
    items: &[Bookmark],
) -> Vec<dash_api::models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner> {
    items
        .iter()
        .map(|item| {
            dash_api::models::GetDashboard200ResponseShelvedCategoriesInnerBookmarksInner::new(
                item.id().as_uuid(),
                item.name().to_owned(),
                item.icon().as_str().to_owned(),
                item.address().as_str().to_owned(),
            )
        })
        .collect()
}
fn category(item: &Category) -> dash_api::models::GetDashboard200ResponseShelvedCategoriesInner {
    dash_api::models::GetDashboard200ResponseShelvedCategoriesInner::new(
        item.id().as_uuid(),
        item.name().to_owned(),
        item.bookmarks_sort_order().as_str().to_owned(),
        bookmarks(item.bookmarks()),
    )
}
fn dashboard(view: DashboardView) -> dash_api::models::GetDashboard200Response {
    let apps = view
        .applications
        .into_iter()
        .map(|app| {
            let mut result = dash_api::models::GetDashboard200ResponseApplicationsInner::new(
                app.id().as_uuid(),
                app.name().to_owned(),
                app.icon().as_str().to_owned(),
                app.address().as_str().to_owned(),
            );
            result.health = app.health().map(|value| {
                match value {
                    crate::catalog::domain::ApplicationHealth::Up => "up",
                    crate::catalog::domain::ApplicationHealth::Down => "down",
                }
                .to_owned()
            });
            result
        })
        .collect();
    dash_api::models::GetDashboard200Response::new(
        apps,
        view.categories_sort_order.as_str().to_owned(),
        view.shelved_categories
            .iter()
            .map(|item| category(item))
            .collect(),
        view.categories.iter().map(category).collect(),
    )
}

async fn get_dashboard(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_read")?;
    let view = crate::dashboard::presentation::http::handlers::get_dashboard_handler::handle(
        api.dashboard,
        api.catalog.clone(),
        api.catalog,
        api.preferences,
        &claims.user_id,
        &claims.groups,
    )
    .await?;
    Ok(error::json(StatusCode::OK, dashboard(view)))
}

async fn create_category(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::CreateCategoryRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    let id = crate::dashboard::presentation::http::handlers::create_category_handler::handle(
        api.dashboard.clone(),
        claims.user_id.clone(),
        &body,
    )
    .await?;
    let item = crate::dashboard::application::queries::GetUserCategory::new(api.dashboard)
        .execute(&claims.user_id, id)
        .await?;
    Ok(error::json(
        StatusCode::CREATED,
        dash_api::models::CreateCategory201Response::new(
            item.id().as_uuid(),
            item.name().to_owned(),
            item.bookmarks_sort_order().as_str().to_owned(),
            bookmarks(item.bookmarks()),
        ),
    ))
}
async fn update_category(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(category_id): Path<Uuid>,
    ApiJson(body): ApiJson<dash_api::models::UpdateCategoryRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    let id = category_id.into();
    crate::dashboard::presentation::http::handlers::update_category_handler::handle(
        api.dashboard.clone(),
        claims.user_id.clone(),
        id,
        &body,
    )
    .await?;
    let item = crate::dashboard::application::queries::GetUserCategory::new(api.dashboard)
        .execute(&claims.user_id, id)
        .await?;
    Ok(error::json(
        StatusCode::OK,
        dash_api::models::CreateCategory201Response::new(
            item.id().as_uuid(),
            item.name().to_owned(),
            item.bookmarks_sort_order().as_str().to_owned(),
            bookmarks(item.bookmarks()),
        ),
    ))
}
async fn delete_category(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(category_id): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::delete_category_handler::handle(
        api.dashboard,
        &claims.user_id,
        category_id.into(),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn create_bookmark(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(category_id): Path<Uuid>,
    ApiJson(body): ApiJson<dash_api::models::CreateBookmarkRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    let id = crate::dashboard::presentation::http::handlers::create_bookmark_handler::handle(
        api.dashboard.clone(),
        claims.user_id.clone(),
        category_id.into(),
        &body,
    )
    .await?;
    let item = crate::dashboard::application::queries::GetUserBookmark::new(api.dashboard)
        .execute(&claims.user_id, id)
        .await?;
    Ok(error::json(
        StatusCode::CREATED,
        dash_api::models::CreateBookmark201Response::new(
            item.id().as_uuid(),
            item.name().to_owned(),
            item.icon().as_str().to_owned(),
            item.address().as_str().to_owned(),
        ),
    ))
}
async fn update_bookmark(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path((category_id, bookmark_id)): Path<(Uuid, Uuid)>,
    ApiJson(body): ApiJson<dash_api::models::CreateBookmarkRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    let id = bookmark_id.into();
    crate::dashboard::presentation::http::handlers::update_bookmark_handler::handle(
        api.dashboard.clone(),
        claims.user_id.clone(),
        category_id.into(),
        id,
        &body,
    )
    .await?;
    let item = crate::dashboard::application::queries::GetUserBookmark::new(api.dashboard)
        .execute(&claims.user_id, id)
        .await?;
    Ok(error::json(
        StatusCode::OK,
        dash_api::models::CreateBookmark201Response::new(
            item.id().as_uuid(),
            item.name().to_owned(),
            item.icon().as_str().to_owned(),
            item.address().as_str().to_owned(),
        ),
    ))
}
async fn delete_bookmark(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path((category_id, bookmark_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::delete_bookmark_handler::handle(
        api.dashboard,
        &claims.user_id,
        category_id.into(),
        bookmark_id.into(),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn record_bookmark_access(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path((category_id, bookmark_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::record_bookmark_access_handler::handle(
        api.dashboard,
        claims.user_id,
        category_id.into(),
        bookmark_id.into(),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn reorder_bookmarks(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(category_id): Path<Uuid>,
    ApiJson(body): ApiJson<dash_api::models::ReorderBookmarksRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::reorder_bookmarks_handler::handle(
        api.dashboard,
        claims.user_id,
        category_id.into(),
        &body,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn reorder_categories(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::ReorderCategoriesRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::reorder_categories_handler::handle(
        api.dashboard,
        claims.user_id,
        &body,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn update_categories_sort_order(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::UpdateCategoriesSortOrderRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_dashboard_write")?;
    crate::dashboard::presentation::http::handlers::update_categories_sort_order_handler::handle(
        api.dashboard,
        claims.user_id,
        &body,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
