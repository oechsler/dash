use super::{Bookmark, CategoryError, CategoryId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    id: CategoryId,
    name: String,
    shelved: bool,
    bookmarks: Vec<Bookmark>,
}

impl Category {
    pub fn new(name: impl Into<String>) -> Result<Self, CategoryError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(CategoryError::EmptyName { value: name });
        }
        Ok(Self {
            id: CategoryId::new(),
            name: trimmed.to_owned(),
            shelved: false,
            bookmarks: Vec::new(),
        })
    }

    pub fn id(&self) -> CategoryId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_shelved(&self) -> bool {
        self.shelved
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
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

    pub fn shelve(&mut self) {
        self.shelved = true;
    }

    pub fn unshelve(&mut self) {
        self.shelved = false;
    }

    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), CategoryError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(CategoryError::EmptyName { value: name });
        }
        self.name = trimmed.to_owned();
        Ok(())
    }
}
