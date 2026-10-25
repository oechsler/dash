use chrono::{DateTime, Utc};

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::shared::domain::CollectionSortOrder;

use super::{Bookmark, CategoryError, CategoryId, DashboardError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    id: CategoryId,
    name: String,
    bookmarks: Vec<Bookmark>,
    bookmarks_sort_order: CollectionSortOrder,
}

impl Category {
    pub fn new(name: impl Into<String>) -> Result<Self, DashboardError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(CategoryError::EmptyName { value: name }.into());
        }
        Ok(Self {
            id: CategoryId::new(),
            name: trimmed.to_owned(),
            bookmarks: Vec::new(),
            bookmarks_sort_order: CollectionSortOrder::Alphabetical,
        })
    }

    pub fn id(&self) -> CategoryId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn bookmarks_sort_order(&self) -> CollectionSortOrder {
        self.bookmarks_sort_order
    }

    pub fn set_bookmarks_sort_order(&mut self, sort_order: CollectionSortOrder) {
        self.bookmarks_sort_order = sort_order;
    }

    pub(super) fn sort_bookmarks_for_display(&mut self) {
        match self.bookmarks_sort_order {
            CollectionSortOrder::Alphabetical => self
                .bookmarks
                .sort_by_key(|bookmark| bookmark.name().to_lowercase()),
            CollectionSortOrder::RecentlyAccessed => self
                .bookmarks
                .sort_by_key(|bookmark| Reverse(bookmark.last_accessed_at())),
            CollectionSortOrder::UserDefined => {}
        }
    }

    pub fn reorder_bookmarks(
        &mut self,
        ordered_ids: &[super::BookmarkId],
    ) -> Result<(), DashboardError> {
        let existing_ids: HashSet<_> = self.bookmarks.iter().map(Bookmark::id).collect();
        let requested_ids: HashSet<_> = ordered_ids.iter().copied().collect();
        if requested_ids.len() != ordered_ids.len() || requested_ids != existing_ids {
            return Err(DashboardError::InvalidBookmarkOrder);
        }
        let positions: HashMap<_, _> = ordered_ids
            .iter()
            .enumerate()
            .map(|(position, id)| (*id, position))
            .collect();
        self.bookmarks
            .sort_by_key(|bookmark| positions[&bookmark.id()]);
        self.bookmarks_sort_order = CollectionSortOrder::UserDefined;
        Ok(())
    }

    pub fn last_accessed_at(&self) -> Option<DateTime<Utc>> {
        self.bookmarks
            .iter()
            .filter_map(Bookmark::last_accessed_at)
            .max()
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.bookmarks.push(bookmark);
    }

    pub(super) fn contains_bookmark(&self, id: super::BookmarkId) -> bool {
        self.bookmarks.iter().any(|bookmark| bookmark.id() == id)
    }

    pub(super) fn bookmark_mut(&mut self, id: super::BookmarkId) -> Option<&mut Bookmark> {
        self.bookmarks
            .iter_mut()
            .find(|bookmark| bookmark.id() == id)
    }

    pub(super) fn take_bookmark(&mut self, id: super::BookmarkId) -> Option<Bookmark> {
        self.bookmarks
            .iter()
            .position(|bookmark| bookmark.id() == id)
            .map(|index| self.bookmarks.remove(index))
    }

    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), DashboardError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(CategoryError::EmptyName { value: name }.into());
        }
        self.name = trimmed.to_owned();
        Ok(())
    }

    pub fn record_bookmark_access(
        &mut self,
        bookmark_id: super::BookmarkId,
        accessed_at: DateTime<Utc>,
    ) -> Result<(), DashboardError> {
        self.bookmark_mut(bookmark_id)
            .ok_or(DashboardError::BookmarkNotFound { bookmark_id })?
            .record_access(accessed_at);
        Ok(())
    }
}
