use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BookmarkId(Uuid);

impl BookmarkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for BookmarkId {
    fn default() -> Self {
        Self::new()
    }
}
