use std::error::Error;

use crate::shared::infrastructure::DatabaseError;

#[test]
fn database_error_keeps_the_original_cause() {
    let error = DatabaseError::source(std::io::Error::other("connection reset"));

    assert_eq!(
        error.source().expect("database cause").to_string(),
        "connection reset"
    );
}

#[test]
fn invalid_stored_values_are_typed_infrastructure_errors() {
    let error = DatabaseError::invalid_value("language", "xx");

    assert!(error.to_string().contains("language"));
    assert!(error.to_string().contains("xx"));
}
