use std::error::Error;

use crate::{
    dashboard::domain::{BookmarkError, DashboardError},
    shared::domain::DomainError,
};

use super::error::ApplicationError;

#[test]
fn domain_failures_keep_the_full_chain_to_application_error() {
    let bookmark_error = BookmarkError::EmptyName {
        value: "   ".to_owned(),
    };
    let domain_error = DomainError::from(DashboardError::from(bookmark_error));
    let application_error = ApplicationError::from(domain_error);

    assert!(
        application_error
            .to_string()
            .contains("dashboard domain error")
    );
    let domain_source = application_error.source().unwrap();
    assert!(
        domain_source
            .to_string()
            .contains("bookmark name must not be empty")
    );
    assert!(domain_source.source().is_some());
}

#[test]
fn repository_failures_retain_the_original_error_as_source() {
    let application_error =
        ApplicationError::repository("load dashboard", std::io::Error::other("connection reset"));

    assert!(application_error.to_string().contains("load dashboard"));
    assert_eq!(
        application_error.source().unwrap().to_string(),
        "connection reset"
    );
}
