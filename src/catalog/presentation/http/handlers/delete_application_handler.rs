use crate::{
    catalog::{
        application::{
            commands::DeleteApplication,
            ports::{ApplicationReader, ApplicationWriter},
        },
        domain::ApplicationId,
    },
    shared::application::ApplicationResult,
};

/// Handles `DELETE /api/v1/applications/{application_id}`.
pub async fn handle<R>(repository: R, id: ApplicationId) -> ApplicationResult<()>
where
    R: ApplicationReader + ApplicationWriter,
{
    DeleteApplication::new(repository).execute(id).await
}
