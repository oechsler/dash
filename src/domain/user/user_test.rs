use super::{GroupId, ProfilePictureUrl, User, UserError, UserId};

#[test]
fn user_contains_provider_independent_profile_and_group_membership() {
    let engineering = GroupId::new("engineering").unwrap();
    let user = User::builder()
        .id(UserId::new("stable-user-identifier").unwrap())
        .username("  samuel  ")
        .first_name(" Samuel ".to_owned())
        .last_name(" Example ".to_owned())
        .profile_picture(ProfilePictureUrl::parse("https://id.example/avatar.png").unwrap())
        .groups([engineering.clone()])
        .build()
        .unwrap();

    assert_eq!(user.id().as_str(), "stable-user-identifier");
    assert_eq!(user.username(), "samuel");
    assert_eq!(user.first_name(), Some("Samuel"));
    assert_eq!(user.last_name(), Some("Example"));
    assert_eq!(
        user.profile_picture().map(ProfilePictureUrl::as_str),
        Some("https://id.example/avatar.png")
    );
    assert!(user.belongs_to(&engineering));
    assert!(!user.belongs_to(&GroupId::new("operations").unwrap()));
}

#[test]
fn user_allows_missing_name_claims_but_requires_a_username() {
    let user = User::builder()
        .id(UserId::new("user-1").unwrap())
        .username("samuel")
        .last_name("   ".to_owned())
        .groups([])
        .build()
        .unwrap();

    assert_eq!(user.first_name(), None);
    assert_eq!(user.last_name(), None);
    assert_eq!(
        User::builder()
            .id(UserId::new("user-1").unwrap())
            .username(" ")
            .groups([])
            .build(),
        Err(UserError::EmptyUsername {
            value: " ".to_owned()
        })
    );
}
