use crate::user::domain::UserId;

use super::{Bookmark, BookmarkId, Category, CategoryId, DashboardError};

/// The personal dashboard aggregate. Its categories and bookmarks are owned by one user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDashboard {
    owner: UserId,
    categories: Vec<Category>,
}

impl UserDashboard {
    pub fn new(owner: UserId) -> Self {
        Self {
            owner,
            categories: Vec::new(),
        }
    }

    pub fn owner(&self) -> &UserId {
        &self.owner
    }

    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    pub fn add_category(&mut self, category: Category) {
        self.categories.push(category);
    }

    pub fn category_mut(&mut self, id: CategoryId) -> Option<&mut Category> {
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
    pub fn remove_category(&mut self, id: CategoryId) -> Result<Category, DashboardError> {
        let index = self
            .categories
            .iter()
            .position(|category| category.id() == id)
            .ok_or(DashboardError::CategoryNotFound { category_id: id })?;
        Ok(self.categories.remove(index))
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
