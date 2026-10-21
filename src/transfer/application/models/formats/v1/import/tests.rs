use crate::{shared::application::ApplicationError, transfer::application::error::TransferError};

use super::signature;

#[test]
fn import_requires_a_signature() {
    let payload = br#"{"version":1,"exported_at":"2026-01-01T00:00:00Z","username":"sam","settings":{"language":"en","timezone":"auto"},"themes":[],"categories":[]}"#;

    assert!(matches!(
        signature::parse(payload),
        Err(ApplicationError::Transfer(TransferError::MissingSignature))
    ));
}
