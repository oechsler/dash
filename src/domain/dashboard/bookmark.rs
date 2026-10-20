use crate::domain::icon::IconReference;

use super::{BookmarkError, BookmarkId, DashboardError, WebAddress};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    id: BookmarkId,
    name: String,
    icon: IconReference,
    address: WebAddress,
}

impl Bookmark {
    pub fn new(
        name: impl Into<String>,
        icon: IconReference,
        address: WebAddress,
    ) -> Result<Self, DashboardError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(BookmarkError::EmptyName { value: name }.into());
        }
        Ok(Self {
            id: BookmarkId::new(),
            name: trimmed.to_owned(),
            icon,
            address,
        })
    }

    pub fn id(&self) -> BookmarkId {
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

    pub fn rename(&mut self, name: impl Into<String>) -> Result<(), DashboardError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(BookmarkError::EmptyName { value: name }.into());
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
}
