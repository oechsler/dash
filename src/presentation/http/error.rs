use axum::extract::FromRequest;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

use crate::shared::application::ApplicationError;

/// HTTP error boundary. Domain/application errors are translated here, once.
#[derive(Debug)]
pub struct HttpError(pub ApplicationError);

impl From<ApplicationError> for HttpError {
    fn from(value: ApplicationError) -> Self {
        Self(value)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, code, title) = match &self.0 {
            ApplicationError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                "not_found",
                "The requested item was not found.",
            ),
            ApplicationError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "You do not have permission to perform this operation.",
            ),
            ApplicationError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication is required.",
            ),
            ApplicationError::BadRequest | ApplicationError::Transfer(_) => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The request is invalid.",
            ),
            ApplicationError::Domain(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                "One or more values are invalid.",
            ),
            ApplicationError::Infrastructure { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "Something went wrong while handling the request.",
            ),
        };
        tracing::error!(error = ?self.0, status = status.as_u16(), "HTTP endpoint failed");
        let body = serde_json::json!({"type":"about:blank", "title":title, "status":status.as_u16(), "code":code, "message":title});
        (
            status,
            [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
            Json(body),
        )
            .into_response()
    }
}

/// Ensures a generated OpenAPI request body also passes its schema validators.
pub fn validate<T: validator::Validate>(value: &T) -> Result<(), HttpError> {
    value
        .validate()
        .map_err(|_| HttpError(ApplicationError::BadRequest))
}

pub fn json<T: serde::Serialize>(status: StatusCode, value: T) -> Response {
    (status, Json(value)).into_response()
}

/// JSON request extractor with contract validation and consistent problem errors.
#[derive(Debug)]
pub struct ApiJson<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + validator::Validate + Send,
{
    type Rejection = HttpError;

    async fn from_request(
        request: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state)
            .await
            .map_err(|_| HttpError(ApplicationError::BadRequest))?;
        validate(&value)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        extract::FromRequest,
        http::{Request, header::CONTENT_TYPE},
        response::IntoResponse,
    };

    use super::{ApiJson, HttpError};

    #[tokio::test]
    async fn malformed_json_uses_the_central_problem_response() {
        let request = Request::builder()
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from("{"))
            .unwrap();

        let error =
            ApiJson::<dash_api::models::CreateApplicationRequest>::from_request(request, &())
                .await
                .unwrap_err();
        let response = HttpError(error.0).into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
        assert_eq!(
            response.headers()[axum::http::header::CONTENT_TYPE],
            "application/problem+json"
        );
    }
}
