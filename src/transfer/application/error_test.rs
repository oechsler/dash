use std::error::Error;

use crate::{shared::application::ApplicationError, transfer::application::error::TransferError};

#[test]
fn transfer_errors_remain_a_distinct_application_error_source() {
    let error = ApplicationError::from(TransferError::InvalidSignature);

    assert!(
        error
            .to_string()
            .contains("user-data export signature is invalid")
    );
    assert!(matches!(
        error.source().unwrap().downcast_ref::<TransferError>(),
        Some(TransferError::InvalidSignature)
    ));
}
