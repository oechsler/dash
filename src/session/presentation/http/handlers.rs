//! Atomic session endpoint handlers.

#[path = "handlers/begin_login_handler.rs"]
pub mod begin_login_handler;
#[path = "handlers/complete_login_handler.rs"]
pub mod complete_login_handler;
#[path = "handlers/downgrade_session_handler.rs"]
pub mod downgrade_session_handler;
#[path = "handlers/invalidate_session_handler.rs"]
pub mod invalidate_session_handler;
#[path = "handlers/list_sessions_handler.rs"]
pub mod list_sessions_handler;
#[path = "handlers/logout_handler.rs"]
pub mod logout_handler;
#[path = "handlers/terminate_session_handler.rs"]
pub mod terminate_session_handler;
#[path = "handlers/upgrade_session_handler.rs"]
pub mod upgrade_session_handler;
