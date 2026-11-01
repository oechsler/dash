use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationId(Uuid);

impl ApplicationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ApplicationId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Default for ApplicationId {
    fn default() -> Self {
        Self::new()
    }
}
