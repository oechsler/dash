mod health_check_probe;

#[cfg(test)]
mod health_check_probe_test;

pub use health_check_probe::{HealthCheckProbeResult, ReqwestHealthCheckProbe};
