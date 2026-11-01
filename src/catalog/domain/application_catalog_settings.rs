use super::DomainResult;
use std::collections::{HashMap, HashSet};

use crate::shared::domain::CollectionSortOrder;

use super::{Application, ApplicationId, CatalogError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCatalogSettings {
    sort_order: CollectionSortOrder,
    manual_order: Vec<ApplicationId>,
}

impl Default for ApplicationCatalogSettings {
    fn default() -> Self {
        Self {
            sort_order: CollectionSortOrder::Alphabetical,
            manual_order: Vec::new(),
        }
    }
}

impl ApplicationCatalogSettings {
    pub fn sort_order(&self) -> CollectionSortOrder {
        self.sort_order
    }

    pub fn manual_order(&self) -> &[ApplicationId] {
        &self.manual_order
    }

    pub fn set_sort_order(&mut self, sort_order: CollectionSortOrder) {
        self.sort_order = sort_order;
    }

    pub fn set_manual_order(
        &mut self,
        ordered_ids: Vec<ApplicationId>,
        available_ids: &[ApplicationId],
    ) -> DomainResult<()> {
        let requested: HashSet<_> = ordered_ids.iter().copied().collect();
        let available: HashSet<_> = available_ids.iter().copied().collect();
        if requested.len() != ordered_ids.len() || requested != available {
            return Err(CatalogError::InvalidApplicationOrder);
        }
        self.manual_order = ordered_ids;
        Ok(())
    }

    pub fn sort_applications(
        &self,
        applications: &mut [Application],
        recently_accessed_ids: &[ApplicationId],
    ) {
        match self.sort_order {
            CollectionSortOrder::Alphabetical => {
                applications.sort_by_key(|app| app.name().to_lowercase());
            }
            CollectionSortOrder::RecentlyAccessed => {
                sort_by_ids(applications, recently_accessed_ids);
            }
            CollectionSortOrder::UserDefined => {
                sort_by_ids(applications, &self.manual_order);
            }
        }
    }
}

fn sort_by_ids(applications: &mut [Application], ordered_ids: &[ApplicationId]) {
    let positions: HashMap<_, _> = ordered_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, index))
        .collect();
    applications.sort_by_key(|application| {
        (
            positions
                .get(&application.id())
                .copied()
                .unwrap_or(usize::MAX),
            application.name().to_lowercase(),
        )
    });
}
