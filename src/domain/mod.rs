mod domain_error;

#[cfg(test)]
mod domain_error_test;

pub mod catalog;
pub mod dashboard;
pub mod icon;
pub mod preferences;
pub mod session;
pub mod theme;
pub mod user;

pub use domain_error::DomainError;
