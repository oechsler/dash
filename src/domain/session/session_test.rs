use std::net::IpAddr;
use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::domain::user::UserId;

use super::{LongLived, Session, SessionError, SessionMetadata, ShortLived, UserSession};

fn at(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .unwrap()
        .with_timezone(&Utc)
}

fn metadata(browser: &str, ip: &str) -> SessionMetadata {
    SessionMetadata::new(
        Some(browser.to_owned()),
        Some("Linux".to_owned()),
        Some(IpAddr::from_str(ip).unwrap()),
    )
}

fn short_session() -> Session<ShortLived> {
    Session::<ShortLived>::new(
        UserId::new("user-1").unwrap(),
        metadata("Firefox", "192.0.2.1"),
        at("2025-01-01T00:00:00Z"),
        at("2025-01-01T01:00:00Z"),
    )
    .unwrap()
}

#[test]
fn short_lived_session_is_owned_by_one_user_and_expires_at_its_deadline() {
    let session = short_session();

    assert_eq!(session.owner().as_str(), "user-1");
    assert!(session.is_active_at(at("2025-01-01T00:59:59Z")));
    assert!(!session.is_active_at(at("2025-01-01T01:00:00Z")));
}

#[test]
fn access_updates_device_metadata_and_last_access_time() {
    let mut session = short_session();
    let accessed_at = at("2025-01-01T00:10:00Z");

    session
        .record_access(metadata("Safari", "2001:db8::1"), accessed_at)
        .unwrap();

    assert_eq!(session.last_accessed_at(), accessed_at);
    assert_eq!(session.metadata().browser(), Some("Safari"));
    assert_eq!(session.metadata().operating_system(), Some("Linux"));
    assert_eq!(
        session.metadata().ip_address().unwrap().to_string(),
        "2001:db8::1"
    );
}

#[test]
fn inactive_session_rejects_access_updates() {
    let mut session = short_session();
    session.terminate(at("2025-01-01T00:05:00Z"));
    let accessed_at = at("2025-01-01T00:10:00Z");

    assert_eq!(
        session.record_access(metadata("Safari", "192.0.2.2"), accessed_at),
        Err(SessionError::InactiveSession { accessed_at })
    );
}

#[test]
fn terminating_a_session_makes_it_inactive() {
    let mut session = short_session();
    let terminated_at = at("2025-01-01T00:30:00Z");

    session.terminate(terminated_at);
    assert!(!session.is_active_at(terminated_at));
    assert_eq!(session.terminated_at(), Some(terminated_at));
    session.terminate(at("2025-01-01T00:40:00Z"));
    assert_eq!(session.terminated_at(), Some(terminated_at));
}

#[test]
fn upgrade_and_downgrade_consume_the_previous_session_type() {
    let now = at("2025-01-01T00:30:00Z");
    let long_expiry = at("2025-02-01T00:00:00Z");
    let long_lived: Session<LongLived> = short_session().upgrade(now, long_expiry).unwrap();
    assert_eq!(long_lived.expires_at(), long_expiry);

    let short_expiry = at("2025-01-01T00:45:00Z");
    let short_lived: Session<ShortLived> = long_lived.downgrade(now, short_expiry).unwrap();
    assert_eq!(short_lived.expires_at(), short_expiry);
}

#[test]
fn runtime_session_enum_represents_a_mixed_session_list() {
    let session = UserSession::ShortLived(short_session());
    let long_lived = session
        .upgrade(at("2025-01-01T00:30:00Z"), at("2025-02-01T00:00:00Z"))
        .unwrap();

    assert!(matches!(long_lived, UserSession::LongLived(_)));
}

#[test]
fn inactive_sessions_cannot_change_lifetime() {
    let now = at("2025-01-01T01:00:00Z");
    let result = short_session().upgrade(now, at("2025-02-01T00:00:00Z"));

    assert_eq!(result, Err(SessionError::CannotChangeInactiveSession));
}

#[test]
fn session_rejects_an_expiry_before_issue_time() {
    let result = Session::<ShortLived>::new(
        UserId::new("user-1").unwrap(),
        SessionMetadata::default(),
        at("2025-01-01T01:00:00Z"),
        at("2025-01-01T00:00:00Z"),
    );

    assert!(matches!(
        result,
        Err(SessionError::ExpiryNotAfterIssue { .. })
    ));
}
