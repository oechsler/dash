use crate::{
    dashboard::{
        application::ports::{DashboardReader, DashboardWriter},
        domain::UserDashboard,
    },
    shared::{application::ApplicationError, domain::CollectionSortOrder},
    user::domain::UserId,
};

pub struct UpdateCategoriesSortOrderInput {
    pub owner: UserId,
    pub sort_order: CollectionSortOrder,
}

pub struct UpdateCategoriesSortOrder<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> UpdateCategoriesSortOrder<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: UpdateCategoriesSortOrderInput,
    ) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner.clone()));
        dashboard.set_categories_sort_order(input.sort_order);
        self.repository.save(&dashboard).await
    }
}
