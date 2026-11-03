//! Atomic endpoint handlers for catalog operations.

#[path = "handlers/create_application_handler.rs"]
pub mod create_application_handler;
#[path = "handlers/delete_application_handler.rs"]
pub mod delete_application_handler;
#[path = "handlers/get_application_handler.rs"]
pub mod get_application_handler;
#[path = "handlers/list_applications_handler.rs"]
pub mod list_applications_handler;
#[path = "handlers/record_application_access_handler.rs"]
pub mod record_application_access_handler;
#[path = "handlers/update_application_catalog_order_handler.rs"]
pub mod update_application_catalog_order_handler;
#[path = "handlers/update_application_catalog_sort_order_handler.rs"]
pub mod update_application_catalog_sort_order_handler;
#[path = "handlers/update_application_handler.rs"]
pub mod update_application_handler;
