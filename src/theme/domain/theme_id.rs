use uuid::Uuid;

use super::GlobalThemeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeId {
    Global(GlobalThemeId),
    UserDefined(Uuid),
}

impl ThemeId {
    pub(super) fn user_defined() -> Self {
        Self::UserDefined(Uuid::new_v4())
    }
}
