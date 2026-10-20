/// Typestate marker for sessions that expire according to the short-lived policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortLived;

/// Typestate marker for sessions that expire according to the long-lived policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongLived;
