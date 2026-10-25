//! Adapters that translate transfer documents to and from application snapshots.
//!
//! HTTP handlers, CLI commands, and file endpoints should use this module at
//! the boundary; application use cases operate only on `UserDataSnapshot`.

pub mod formats;
