use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use chrono::{DateTime, Utc};

use crate::{shared::domain::CollectionSortOrder, user::domain::UserId};

use super::{Bookmark, BookmarkId, CategoryId, DashboardCategory, DashboardError};

/// The personal dashboard aggregate. Its categories and bookmarks are owned by one user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDashboard {
    owner: UserId,
    categories: Vec<DashboardCategory>,
    categories_sort_order: CollectionSortOrder,
}

impl UserDashboard {
    pub fn new(owner: UserId) -> Self {
        Self {
            owner,
            categories: Vec::new(),
            categories_sort_order: CollectionSortOrder::Alphabetical,
        }
    }

    pub fn owner(&self) -> &UserId {
        &self.owner
    }

    pub fn categories(&self) -> &[DashboardCategory] {
        &self.categories
    }

    pub fn categories_sort_order(&self) -> CollectionSortOrder {
        self.categories_sort_order
    }

    pub fn set_categories_sort_order(&mut self, sort_order: CollectionSortOrder) {
        self.categories_sort_order = sort_order;
    }

    pub fn reorder_categories(&mut self, ordered_ids: &[CategoryId]) -> Result<(), DashboardError> {
        let existing_ids: HashSet<_> = self.categories.iter().map(DashboardCategory::id).collect();
        let requested_ids: HashSet<_> = ordered_ids.iter().copied().collect();
        if requested_ids.len() != ordered_ids.len() || requested_ids != existing_ids {
            return Err(DashboardError::InvalidCategoryOrder);
        }

        let positions: HashMap<_, _> = ordered_ids
            .iter()
            .enumerate()
            .map(|(position, id)| (*id, position))
            .collect();
        self.categories
            .sort_by_key(|category| positions[&category.id()]);
        Ok(())
    }

    pub fn sort_bookmarks_for_display(&mut self) {
        for category in &mut self.categories {
            category.sort_bookmarks_for_display();
        }
    }

    pub fn sort_categories_for_display(&mut self) {
        match self.categories_sort_order {
            CollectionSortOrder::Alphabetical => self
                .categories
                .sort_by_key(|category| category.name().to_lowercase()),
            CollectionSortOrder::RecentlyAccessed => self
                .categories
                .sort_by_key(|category| Reverse(category.last_accessed_at())),
            CollectionSortOrder::UserDefined => {}
        }
    }

    pub fn record_bookmark_access(
        &mut self,
        category_id: CategoryId,
        bookmark_id: BookmarkId,
        accessed_at: DateTime<Utc>,
    ) -> Result<(), DashboardError> {
        let category = self
            .category_mut(category_id)
            .ok_or(DashboardError::CategoryNotFound { category_id })?;
        category.record_bookmark_access(bookmark_id, accessed_at)
    }

    pub fn add_category(&mut self, category: impl Into<DashboardCategory>) {
        self.categories.push(category.into());
    }

    pub fn category_mut(&mut self, id: CategoryId) -> Option<&mut DashboardCategory> {
        self.categories
            .iter_mut()
            .find(|category| category.id() == id)
    }

    pub fn rename_category(
        &mut self,
        id: CategoryId,
        name: impl Into<String>,
    ) -> Result<(), DashboardError> {
        self.category_mut(id)
            .ok_or(DashboardError::CategoryNotFound { category_id: id })?
            .rename(name)?;
        Ok(())
    }

    /// Removes a category and returns it with its bookmarks for cascading persistence.
    pub fn remove_category(&mut self, id: CategoryId) -> Result<DashboardCategory, DashboardError> {
        let index = self
            .categories
            .iter()
            .position(|category| category.id() == id)
            .ok_or(DashboardError::CategoryNotFound { category_id: id })?;
        Ok(self.categories.remove(index))
    }

    pub fn shelve_category(&mut self, id: CategoryId) -> Result<(), DashboardError> {
        self.transition_category(id, DashboardCategory::shelve)
    }

    pub fn unshelve_category(&mut self, id: CategoryId) -> Result<(), DashboardError> {
        self.transition_category(id, DashboardCategory::unshelve)
    }

    fn transition_category(
        &mut self,
        id: CategoryId,
        transition: fn(DashboardCategory) -> DashboardCategory,
    ) -> Result<(), DashboardError> {
        let index = self
            .categories
            .iter()
            .position(|category| category.id() == id)
            .ok_or(DashboardError::CategoryNotFound { category_id: id })?;
        let category = transition(self.categories.remove(index));
        self.categories.insert(index, category);
        Ok(())
    }

    pub fn add_bookmark(
        &mut self,
        category_id: CategoryId,
        bookmark: Bookmark,
    ) -> Result<(), DashboardError> {
        let category = self
            .category_mut(category_id)
            .ok_or(DashboardError::CategoryNotFound { category_id })?;
        category.add_bookmark(bookmark);
        Ok(())
    }

    pub fn bookmark_mut(&mut self, id: BookmarkId) -> Result<&mut Bookmark, DashboardError> {
        self.categories
            .iter_mut()
            .find_map(|category| category.bookmark_mut(id))
            .ok_or(DashboardError::BookmarkNotFound { bookmark_id: id })
    }

    pub fn rename_bookmark(
        &mut self,
        id: BookmarkId,
        name: impl Into<String>,
    ) -> Result<(), DashboardError> {
        self.bookmark_mut(id)?.rename(name)?;
        Ok(())
    }

    pub fn remove_bookmark(&mut self, id: BookmarkId) -> Result<Bookmark, DashboardError> {
        self.categories
            .iter_mut()
            .find_map(|category| category.take_bookmark(id))
            .ok_or(DashboardError::BookmarkNotFound { bookmark_id: id })
    }

    pub fn move_bookmark(
        &mut self,
        bookmark_id: BookmarkId,
        target_category_id: CategoryId,
    ) -> Result<(), DashboardError> {
        let target_index = self
            .categories
            .iter()
            .position(|category| category.id() == target_category_id)
            .ok_or(DashboardError::CategoryNotFound {
                category_id: target_category_id,
            })?;
        let source_index = self
            .categories
            .iter()
            .position(|category| category.contains_bookmark(bookmark_id))
            .ok_or(DashboardError::BookmarkNotFound { bookmark_id })?;

        if source_index == target_index {
            return Ok(());
        }

        let bookmark = self.categories[source_index]
            .take_bookmark(bookmark_id)
            .ok_or(DashboardError::BookmarkNotFound { bookmark_id })?;
        self.categories[target_index].add_bookmark(bookmark);
        Ok(())
    }
}
