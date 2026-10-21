use std::ops::{Deref, DerefMut};

use super::Category;

/// A category that is archived and hidden from the active dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShelvedCategory(Category);

impl ShelvedCategory {
    pub fn unshelve(self) -> Category {
        self.0
    }
}

impl From<Category> for ShelvedCategory {
    fn from(category: Category) -> Self {
        Self(category)
    }
}

impl Deref for ShelvedCategory {
    type Target = Category;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShelvedCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
