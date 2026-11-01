use super::{IconError, IconReference, IconReferenceError};

#[test]
fn icon_reference_accepts_arbitrary_namespaces_and_urls() {
    assert_eq!(
        IconReference::try_from("custom-pack:star")
            .unwrap()
            .as_str(),
        "custom-pack:star"
    );
    assert_eq!(
        IconReference::try_from("https://assets.example/icon.svg")
            .unwrap()
            .as_str(),
        "https://assets.example/icon.svg"
    );
}

#[test]
fn icon_reference_rejects_only_empty_values() {
    assert_eq!(
        IconReference::try_from("  "),
        Err(IconError::InvalidReference(IconReferenceError::Empty {
            value: "  ".to_owned()
        }))
    );
}
