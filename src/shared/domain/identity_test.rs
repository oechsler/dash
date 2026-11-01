use super::{GroupId, IdentityError, UserId};

#[test]
fn user_id_rejects_empty_or_whitespace_values() {
    assert_eq!(
        UserId::try_from(" \t"),
        Err(IdentityError::EmptyUserId {
            value: " \t".to_owned(),
        })
    );
}

#[test]
fn group_id_rejects_empty_values_and_preserves_valid_provider_ids() {
    assert!(matches!(
        GroupId::try_from(String::new()),
        Err(IdentityError::EmptyGroupId { value }) if value.is_empty()
    ));
    let group = GroupId::try_from(" engineering ").unwrap();
    assert_eq!(group.as_str(), " engineering ");
}
