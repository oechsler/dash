use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CategoryId(Uuid);

impl CategoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CategoryId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Default for CategoryId {
    fn default() -> Self {
        Self::new()
    }
}
