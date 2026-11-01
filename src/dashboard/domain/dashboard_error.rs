use thiserror::Error;

use super::{BookmarkError, BookmarkId, CategoryError, CategoryId};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DashboardError {
    #[error("dashboard category operation failed: {0}")]
    Category(#[from] CategoryError),

    #[error("dashboard bookmark operation failed: {0}")]
    Bookmark(#[from] BookmarkError),

    #[error("category {category_id:?} does not belong to this dashboard")]
    CategoryNotFound { category_id: CategoryId },

    #[error("bookmark {bookmark_id:?} does not belong to this dashboard")]
    BookmarkNotFound { bookmark_id: BookmarkId },

    #[error("category order must contain every dashboard category exactly once")]
    InvalidCategoryOrder,
}

/// Result for operations that coordinate categories and bookmarks in a dashboard.
pub type DomainResult<T> = Result<T, DashboardError>;
