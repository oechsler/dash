mod bookmark;
mod bookmark_id;
mod category;
mod category_id;
mod dashboard_category;
mod dashboard_error;
mod shelved_category;
mod user_dashboard;

#[cfg(test)]
mod bookmark_test;
#[cfg(test)]
mod category_test;
#[cfg(test)]
mod user_dashboard_test;

pub use crate::shared::domain::{WebAddress, WebAddressError};
pub use bookmark::{Bookmark, BookmarkError, BookmarkResult};
pub use bookmark_id::BookmarkId;
pub use category::{Category, CategoryError, CategoryResult};
pub use category_id::CategoryId;
pub use dashboard_category::DashboardCategory;
pub use dashboard_error::{DashboardError, DomainResult};
pub use shelved_category::ShelvedCategory;
pub use user_dashboard::UserDashboard;
