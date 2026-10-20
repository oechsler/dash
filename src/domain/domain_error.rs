use thiserror::Error;

use crate::domain::{
    catalog::CatalogError, dashboard::DashboardError, icon::IconError,
    preferences::PreferencesError, session::SessionError, theme::ThemeError, user::UserError,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("dashboard domain error: {0}")]
    Dashboard(#[from] DashboardError),

    #[error("application catalog domain error: {0}")]
    Catalog(#[from] CatalogError),

    #[error("icon domain error: {0}")]
    Icon(#[from] IconError),

    #[error("user preferences domain error: {0}")]
    Preferences(#[from] PreferencesError),

    #[error("session domain error: {0}")]
    Session(#[from] SessionError),

    #[error("theme domain error: {0}")]
    Theme(#[from] ThemeError),

    #[error("user domain error: {0}")]
    User(#[from] UserError),
}
