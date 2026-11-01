//! Domain-oriented application for managing dashboards, shared applications,
//! preferences, themes, sessions, users, and portable user-data backups.
//!
//! Each business context keeps its domain model separate from application
//! use-cases and infrastructure adapters. [`shared`] contains only concepts
//! intentionally used across those contexts; adapters implement application
//! ports rather than leaking persistence details into domain code.

pub mod catalog;
pub mod composition;
pub mod dashboard;
pub mod icon;
pub mod preferences;
pub mod session;
pub mod shared;
pub mod theme;
pub mod transfer;
pub mod user;
