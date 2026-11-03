//! Identity-provider integration used by the HTTP boundary.

mod oidc;

pub use oidc::{OidcConfig, OidcIdentity, OidcProvider, OidcProviderError};
