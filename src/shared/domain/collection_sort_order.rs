#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CollectionSortOrder {
    #[default]
    Alphabetical,
    RecentlyAccessed,
    UserDefined,
}

/// Result for parsing collection sort orders.
pub type CollectionSortOrderResult<T> = Result<T, &'static str>;

impl CollectionSortOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Alphabetical => "alphabetical",
            Self::RecentlyAccessed => "recently_accessed",
            Self::UserDefined => "user_defined",
        }
    }
}

impl TryFrom<&str> for CollectionSortOrder {
    type Error = &'static str;

    fn try_from(value: &str) -> CollectionSortOrderResult<Self> {
        match value {
            "alphabetical" => Ok(Self::Alphabetical),
            "recently_accessed" => Ok(Self::RecentlyAccessed),
            "user_defined" => Ok(Self::UserDefined),
            _ => Err("unknown collection sort order"),
        }
    }
}
