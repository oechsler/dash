use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionMetadata {
    browser: Option<String>,
    operating_system: Option<String>,
    ip_address: Option<IpAddr>,
}

impl SessionMetadata {
    pub fn new(
        browser: Option<String>,
        operating_system: Option<String>,
        ip_address: Option<IpAddr>,
    ) -> Self {
        Self {
            browser: normalize(browser),
            operating_system: normalize(operating_system),
            ip_address,
        }
    }

    pub fn browser(&self) -> Option<&str> {
        self.browser.as_deref()
    }

    pub fn operating_system(&self) -> Option<&str> {
        self.operating_system.as_deref()
    }

    pub fn ip_address(&self) -> Option<IpAddr> {
        self.ip_address
    }
}

fn normalize(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    })
}
