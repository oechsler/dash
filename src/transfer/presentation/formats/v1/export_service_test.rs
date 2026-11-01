use crate::{
    shared::application::ApplicationError,
    shared::domain::UserId,
    transfer::{
        application::{TransferError, TransferOperation, models::UserDataSnapshot},
        presentation::formats::TransferFormat,
    },
};

use super::V1Format;

#[test]
fn v1_rejects_export_with_an_explicit_unsupported_operation_error() {
    let result = V1Format.export(
        UserDataSnapshot::default(),
        &UserId::try_from("user-1").unwrap(),
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
