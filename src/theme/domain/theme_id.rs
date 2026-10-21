use uuid::Uuid;

use super::GlobalThemeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeId {
    Global(GlobalThemeId),
    UserDefined(Uuid),
}
