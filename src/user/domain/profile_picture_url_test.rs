use super::{ProfilePictureUrl, ProfilePictureUrlError};

#[test]
fn profile_picture_url_accepts_http_and_https() {
    assert_eq!(
        ProfilePictureUrl::try_from("https://identity.example/avatar.png")
            .unwrap()
            .as_str(),
        "https://identity.example/avatar.png"
    );
}

#[test]
fn profile_picture_url_rejects_non_web_schemes() {
    assert!(matches!(
        ProfilePictureUrl::try_from("file:///tmp/avatar.png"),
        Err(ProfilePictureUrlError::InvalidUrl { .. })
    ));
}
