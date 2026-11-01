mod dashboard_repository;
pub(crate) mod entities;
mod migrations;

pub use dashboard_repository::DashboardRepository;
pub use migrations::DashboardSchemaV1;
