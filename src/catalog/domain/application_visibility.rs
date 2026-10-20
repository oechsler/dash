use std::collections::HashSet;

use crate::user::domain::GroupId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationVisibility(Audience);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Audience {
    Everyone,
    Groups(HashSet<GroupId>),
}

impl ApplicationVisibility {
    pub fn everyone() -> Self {
        Self(Audience::Everyone)
    }

    /// An empty group set means unrestricted visibility, matching the domain rule.
    pub fn groups(groups: impl IntoIterator<Item = GroupId>) -> Self {
        let groups: HashSet<_> = groups.into_iter().collect();
        if groups.is_empty() {
            Self::everyone()
        } else {
            Self(Audience::Groups(groups))
        }
    }

    pub fn allows(&self, user_groups: &HashSet<GroupId>) -> bool {
        match &self.0 {
            Audience::Everyone => true,
            Audience::Groups(visible_groups) => !visible_groups.is_disjoint(user_groups),
        }
    }

    pub fn visible_to_groups(&self) -> Option<&HashSet<GroupId>> {
        match &self.0 {
            Audience::Everyone => None,
            Audience::Groups(groups) => Some(groups),
        }
    }
}
