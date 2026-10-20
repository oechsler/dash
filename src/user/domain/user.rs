use std::collections::HashSet;

use bon::bon;
use thiserror::Error;

use super::{GroupId, IdentifierError, ProfilePictureUrl, ProfilePictureUrlError, UserId};

/// Provider-independent user information used by the Dash domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    profile_picture: Option<ProfilePictureUrl>,
    groups: HashSet<GroupId>,
}

#[bon]
impl User {
    #[builder]
    pub fn new(
        id: UserId,
        username: impl Into<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        profile_picture: Option<ProfilePictureUrl>,
        groups: impl IntoIterator<Item = GroupId>,
    ) -> Result<Self, UserError> {
        let username = username.into();
        let trimmed_username = username.trim();
        if trimmed_username.is_empty() {
            return Err(UserError::EmptyUsername { value: username });
        }

        Ok(Self {
            id,
            username: trimmed_username.to_owned(),
            first_name: normalize_optional_name(first_name),
            last_name: normalize_optional_name(last_name),
            profile_picture,
            groups: groups.into_iter().collect(),
        })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    pub fn profile_picture(&self) -> Option<&ProfilePictureUrl> {
        self.profile_picture.as_ref()
    }

    pub fn groups(&self) -> &HashSet<GroupId> {
        &self.groups
    }

    pub fn belongs_to(&self, group: &GroupId) -> bool {
        self.groups.contains(group)
    }
}

fn normalize_optional_name(name: Option<String>) -> Option<String> {
    name.and_then(|name| {
        let trimmed = name.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserError {
    #[error("user identifier is invalid: {0}")]
    Identifier(#[from] IdentifierError),

    #[error("user profile picture is invalid: {0}")]
    ProfilePicture(#[from] ProfilePictureUrlError),

    #[error("username must not be empty (received {value:?})")]
    EmptyUsername { value: String },
}
