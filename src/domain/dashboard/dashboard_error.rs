use thiserror::Error;

use super::{BookmarkError, BookmarkId, CategoryError, CategoryId, WebAddressError};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DashboardError {
    #[error("dashboard category operation failed: {0}")]
    Category(#[from] CategoryError),

    #[error("dashboard bookmark operation failed: {0}")]
    Bookmark(#[from] BookmarkError),

    #[error("dashboard web address validation failed: {0}")]
    WebAddress(#[from] WebAddressError),

    #[error("category {category_id:?} does not belong to this dashboard")]
    CategoryNotFound { category_id: CategoryId },

    #[error("bookmark {bookmark_id:?} does not belong to this dashboard")]
    BookmarkNotFound { bookmark_id: BookmarkId },
}
