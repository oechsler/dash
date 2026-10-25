use chrono::{DateTime, Utc};

use crate::shared::domain::CollectionSortOrder;

use super::{Category, CategoryId, ShelvedCategory};

/// A dashboard category in exactly one of its lifecycle states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DashboardCategory {
    Active(Category),
    Shelved(ShelvedCategory),
}

impl DashboardCategory {
    pub fn id(&self) -> CategoryId {
        match self {
            Self::Active(category) => category.id(),
            Self::Shelved(category) => category.id(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Active(category) => category.name(),
            Self::Shelved(category) => category.name(),
        }
    }

    pub fn bookmarks(&self) -> &[super::Bookmark] {
        match self {
            Self::Active(category) => category.bookmarks(),
            Self::Shelved(category) => category.bookmarks(),
        }
    }

    pub fn last_accessed_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Active(category) => category.last_accessed_at(),
            Self::Shelved(category) => category.last_accessed_at(),
        }
    }

    pub fn bookmarks_sort_order(&self) -> CollectionSortOrder {
        match self {
            Self::Active(category) => category.bookmarks_sort_order(),
            Self::Shelved(category) => category.bookmarks_sort_order(),
        }
    }

    pub(super) fn sort_bookmarks_for_display(&mut self) {
        match self {
            Self::Active(category) => category.sort_bookmarks_for_display(),
            Self::Shelved(category) => category.sort_bookmarks_for_display(),
        }
    }

    pub fn set_bookmarks_sort_order(&mut self, sort_order: CollectionSortOrder) {
        match self {
            Self::Active(category) => category.set_bookmarks_sort_order(sort_order),
            Self::Shelved(category) => category.set_bookmarks_sort_order(sort_order),
        }
    }

    pub fn reorder_bookmarks(
        &mut self,
        ordered_ids: &[super::BookmarkId],
    ) -> Result<(), super::DashboardError> {
        match self {
            Self::Active(category) => category.reorder_bookmarks(ordered_ids),
            Self::Shelved(category) => category.reorder_bookmarks(ordered_ids),
        }
    }

    pub fn as_active(&self) -> Option<&Category> {
        match self {
            Self::Active(category) => Some(category),
            Self::Shelved(_) => None,
        }
    }

    pub fn as_shelved(&self) -> Option<&ShelvedCategory> {
        match self {
            Self::Active(_) => None,
            Self::Shelved(category) => Some(category),
        }
    }

    pub(super) fn rename(&mut self, name: impl Into<String>) -> Result<(), super::DashboardError> {
        match self {
            Self::Active(category) => category.rename(name),
            Self::Shelved(category) => category.rename(name),
        }
    }

    pub(super) fn add_bookmark(&mut self, bookmark: super::Bookmark) {
        match self {
            Self::Active(category) => category.add_bookmark(bookmark),
            Self::Shelved(category) => category.add_bookmark(bookmark),
        }
    }

    pub(super) fn record_bookmark_access(
        &mut self,
        id: super::BookmarkId,
        accessed_at: DateTime<Utc>,
    ) -> Result<(), super::DashboardError> {
        match self {
            Self::Active(category) => category.record_bookmark_access(id, accessed_at),
            Self::Shelved(category) => category.record_bookmark_access(id, accessed_at),
        }
    }

    pub(super) fn bookmark_mut(&mut self, id: super::BookmarkId) -> Option<&mut super::Bookmark> {
        match self {
            Self::Active(category) => category.bookmark_mut(id),
            Self::Shelved(category) => category.bookmark_mut(id),
        }
    }

    pub(super) fn contains_bookmark(&self, id: super::BookmarkId) -> bool {
        match self {
            Self::Active(category) => category.contains_bookmark(id),
            Self::Shelved(category) => category.contains_bookmark(id),
        }
    }

    pub(super) fn take_bookmark(&mut self, id: super::BookmarkId) -> Option<super::Bookmark> {
        match self {
            Self::Active(category) => category.take_bookmark(id),
            Self::Shelved(category) => category.take_bookmark(id),
        }
    }

    pub fn shelve(self) -> Self {
        match self {
            Self::Active(category) => Self::Shelved(category.into()),
            shelved @ Self::Shelved(_) => shelved,
        }
    }

    pub fn unshelve(self) -> Self {
        match self {
            Self::Active(category) => Self::Active(category),
            Self::Shelved(category) => Self::Active(category.unshelve()),
        }
    }
}

impl From<Category> for DashboardCategory {
    fn from(category: Category) -> Self {
        Self::Active(category)
    }
}

impl From<ShelvedCategory> for DashboardCategory {
    fn from(category: ShelvedCategory) -> Self {
        Self::Shelved(category)
    }
}
