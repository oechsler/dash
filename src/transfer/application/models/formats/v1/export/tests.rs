use crate::{
    shared::application::ApplicationError,
    transfer::application::{
        error::{TransferError, TransferOperation},
        models::{UserDataSnapshot, formats::TransferFormat},
    },
    user::domain::UserId,
};

use super::super::V1Format;

#[test]
fn v1_rejects_export_with_an_explicit_unsupported_operation_error() {
    let result = V1Format.export(
        UserDataSnapshot::default(),
        &UserId::new("user-1").unwrap(),
        "sam".to_owned(),
        false,
    );

    assert!(matches!(
        result,
        Err(ApplicationError::Transfer(
            TransferError::UnsupportedOperation {
                version: 1,
                operation: TransferOperation::Export,
            }
        ))
    ));
}
