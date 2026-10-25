use crate::{shared::application::ApplicationError, transfer::application::TransferError};

use super::import_service::ImportService;

#[test]
fn import_requires_a_signature() {
    let payload = br#"{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"sam","settings":{"language":"en","timezone":"auto"},"themes":[],"categories":[]}"#;

    assert!(matches!(
        ImportService.import(
            payload,
            &crate::user::domain::UserId::new("user-1").unwrap()
        ),
        Err(ApplicationError::Transfer(TransferError::MissingSignature))
    ));
}
