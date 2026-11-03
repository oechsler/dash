use serde_json::json;

use super::value_list;

#[test]
fn claim_lists_accept_space_separated_strings_and_string_arrays() {
    assert_eq!(
        value_list(&json!("dash_user_read dash_themes_read")),
        Some(vec!["dash_user_read".into(), "dash_themes_read".into()])
    );
    assert_eq!(
        value_list(&json!(["family", "admins"])),
        Some(vec!["family".into(), "admins".into()])
    );
}

#[test]
fn unsupported_claim_types_are_ignored() {
    assert_eq!(value_list(&json!({"group": "admins"})), None);
    assert_eq!(value_list(&json!(null)), None);
}
