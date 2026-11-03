//! Atomic endpoint handlers for dashboard operations.

#[path = "handlers/create_bookmark_handler.rs"]
pub mod create_bookmark_handler;
#[path = "handlers/create_category_handler.rs"]
pub mod create_category_handler;
#[path = "handlers/delete_bookmark_handler.rs"]
pub mod delete_bookmark_handler;
#[path = "handlers/delete_category_handler.rs"]
pub mod delete_category_handler;
#[path = "handlers/get_dashboard_handler.rs"]
pub mod get_dashboard_handler;
#[path = "handlers/record_bookmark_access_handler.rs"]
pub mod record_bookmark_access_handler;
#[path = "handlers/reorder_bookmarks_handler.rs"]
pub mod reorder_bookmarks_handler;
#[path = "handlers/reorder_categories_handler.rs"]
pub mod reorder_categories_handler;
#[path = "handlers/update_bookmark_handler.rs"]
pub mod update_bookmark_handler;
#[path = "handlers/update_categories_sort_order_handler.rs"]
pub mod update_categories_sort_order_handler;
#[path = "handlers/update_category_handler.rs"]
pub mod update_category_handler;
