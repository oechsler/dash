#[derive(Debug, Clone, Default)]
pub struct UserDataSnapshot {
    pub dashboard: Option<crate::dashboard::domain::UserDashboard>,
    pub preferences: Option<crate::preferences::domain::UserPreferences>,
    pub themes: Vec<crate::theme::domain::Theme>,
    pub applications: Vec<crate::catalog::domain::Application>,
}
