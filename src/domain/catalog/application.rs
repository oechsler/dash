use std::collections::HashSet;

use bon::bon;
use getset::Getters;

use crate::domain::{dashboard::WebAddress, icon::IconReference, user::GroupId};

use super::{ApplicationError, ApplicationId, ApplicationVisibility};

/// An administrator-maintained application in the shared catalog.
#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct Application {
    id: ApplicationId,
    name: String,
    icon: IconReference,
    address: WebAddress,
    #[getset(get = "pub")]
    visibility: ApplicationVisibility,
}

#[bon]
impl Application {
    #[builder]
    pub fn new(
        name: impl Into<String>,
        icon: IconReference,
        address: WebAddress,
        visibility: ApplicationVisibility,
    ) -> Result<Self, ApplicationError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(ApplicationError::EmptyName { value: name });
        }

        Ok(Self {
            id: ApplicationId::new(),
            name: trimmed.to_owned(),
            icon,
            address,
            visibility,
        })
    }

    pub fn id(&self) -> ApplicationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &WebAddress {
        &self.address
    }

    pub fn icon(&self) -> &IconReference {
        &self.icon
    }

    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), ApplicationError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(ApplicationError::EmptyName { value: name });
        }
        self.name = trimmed.to_owned();
        Ok(())
    }

    pub fn change_icon(&mut self, icon: IconReference) {
        self.icon = icon;
    }

    pub fn change_address(&mut self, address: WebAddress) {
        self.address = address;
    }

    pub fn is_visible_to(&self, groups: &HashSet<GroupId>) -> bool {
        self.visibility.allows(groups)
    }

    pub fn restrict_to(&mut self, groups: impl IntoIterator<Item = GroupId>) {
        self.visibility = ApplicationVisibility::groups(groups);
    }
}
