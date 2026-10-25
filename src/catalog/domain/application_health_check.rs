use super::HealthCheckUrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExpectedStatusClass {
    #[default]
    Success2xx,
    Redirect3xx,
    ClientError4xx,
}

impl ExpectedStatusClass {
    pub fn accepts(self, status: u16) -> bool {
        match self {
            Self::Success2xx => (200..300).contains(&status),
            Self::Redirect3xx => (300..400).contains(&status),
            Self::ClientError4xx => (400..500).contains(&status),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Success2xx => "2xx",
            Self::Redirect3xx => "3xx",
            Self::ClientError4xx => "4xx",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationHealthCheck {
    url: Option<HealthCheckUrl>,
    expected_status_class: ExpectedStatusClass,
    ignore_tls_errors: bool,
}

impl ApplicationHealthCheck {
    pub fn new(
        url: Option<HealthCheckUrl>,
        expected_status_class: ExpectedStatusClass,
        ignore_tls_errors: bool,
    ) -> Self {
        Self {
            url,
            expected_status_class,
            ignore_tls_errors,
        }
    }

    pub fn url(&self) -> Option<&HealthCheckUrl> {
        self.url.as_ref()
    }

    pub fn expected_status_class(&self) -> ExpectedStatusClass {
        self.expected_status_class
    }

    pub fn ignore_tls_errors(&self) -> bool {
        self.ignore_tls_errors
    }
}
