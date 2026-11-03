use crate::{
    shared::{application::ApplicationResult, domain::UserId},
    user::application::{commands::DeleteUserData, ports::UserWriter},
};

/// Handles `DELETE /api/v1/me`.
pub async fn handle<R: UserWriter>(users: R, user_id: &UserId) -> ApplicationResult<()> {
    DeleteUserData::new(users).execute(user_id).await
}
