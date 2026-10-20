use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplicationId(Uuid);

impl ApplicationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ApplicationId {
    fn default() -> Self {
        Self::new()
    }
}
