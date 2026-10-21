use crate::{
    transfer::application::models::{UserDataSnapshot, formats::TransferFormat},
    user::domain::UserId,
};

use super::super::V2Format;

#[test]
fn v2_supports_import_and_export_through_the_shared_format_contract() {
    let format = V2Format;
    let owner = UserId::new("user-1").unwrap();
    let payload = format
        .export(UserDataSnapshot::default(), &owner, "sam".to_owned(), false)
        .unwrap();

    let imported = format.import(&payload, &owner).unwrap();
    assert!(imported.dashboard.is_some());
    assert!(imported.preferences.is_some());
    assert!(imported.themes.is_empty());
    assert!(imported.applications.is_empty());
}
