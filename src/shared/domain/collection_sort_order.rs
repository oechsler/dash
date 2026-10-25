#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CollectionSortOrder {
    #[default]
    Alphabetical,
    RecentlyAccessed,
    UserDefined,
}

impl CollectionSortOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alphabetical => "alphabetical",
            Self::RecentlyAccessed => "recently_accessed",
            Self::UserDefined => "user_defined",
        }
    }
}
