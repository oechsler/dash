use crate::user::domain::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeScope {
    Global,
    UserDefined(UserId),
}

impl ThemeScope {
    pub fn is_available_to(&self, user_id: &UserId) -> bool {
        match self {
            Self::Global => true,
            Self::UserDefined(owner) => owner == user_id,
        }
    }

    pub fn is_user_defined(&self) -> bool {
        matches!(self, Self::UserDefined(_))
    }
}
