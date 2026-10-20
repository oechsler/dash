use super::{ProfilePictureUrl, ProfilePictureUrlError, UserError};

#[test]
fn profile_picture_url_accepts_http_and_https() {
    assert_eq!(
        ProfilePictureUrl::parse("https://identity.example/avatar.png")
            .unwrap()
            .as_str(),
        "https://identity.example/avatar.png"
    );
}

#[test]
fn profile_picture_url_rejects_non_web_schemes() {
    assert!(matches!(
        ProfilePictureUrl::parse("file:///tmp/avatar.png"),
        Err(UserError::ProfilePicture(
            ProfilePictureUrlError::InvalidUrl { .. }
        ))
    ));
}
