#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationHealth {
    Up,
    Down,
}

impl ApplicationHealth {
    pub fn from_response(
        http_status: Option<u16>,
        expected_status_class: super::ExpectedStatusClass,
    ) -> Self {
        if http_status.is_some_and(|status| expected_status_class.accepts(status)) {
            Self::Up
        } else {
            Self::Down
        }
    }
}
