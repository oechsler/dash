use uuid::Uuid;

use crate::shared::domain::UserId;

use super::{GlobalThemeId, ThemeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeIdentity {
    Global(GlobalThemeId),
    UserDefined { id: Uuid, owner: UserId },
}

impl ThemeIdentity {
    pub(super) fn user_defined(owner: UserId) -> Self {
        Self::UserDefined {
            id: Uuid::new_v4(),
            owner,
        }
    }

    pub(crate) fn restore_user_defined(id: Uuid, owner: UserId) -> Self {
        Self::UserDefined { id, owner }
    }

    pub fn id(&self) -> ThemeId {
        match self {
            Self::Global(id) => ThemeId::Global(id.clone()),
            Self::UserDefined { id, .. } => ThemeId::UserDefined(*id),
        }
    }

    pub fn is_available_to(&self, user_id: &UserId) -> bool {
        match self {
            Self::Global(_) => true,
            Self::UserDefined { owner, .. } => owner == user_id,
        }
    }

    pub fn is_user_defined(&self) -> bool {
        matches!(self, Self::UserDefined { .. })
    }
}
