use crate::{
    catalog::{
        application::{ports::ApplicationReader, queries::GetApplication},
        domain::{Application, ApplicationId},
    },
    shared::application::ApplicationResult,
};

/// Handles `GET /api/v1/applications/{application_id}`.
pub async fn handle<R: ApplicationReader>(
    repository: R,
    id: ApplicationId,
) -> ApplicationResult<Application> {
    GetApplication::new(repository).execute(id).await
}
