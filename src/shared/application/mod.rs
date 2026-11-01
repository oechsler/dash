mod domain_error;
mod error;
mod port_error;

#[cfg(test)]
mod domain_error_test;
#[cfg(test)]
mod error_test;

pub use domain_error::ApplicationDomainError;
pub use error::{ApplicationError, ApplicationResult};
pub use port_error::{ApplicationPortError, PortResult};
