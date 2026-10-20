use super::{IconReference, IconReferenceError};

#[test]
fn icon_reference_accepts_arbitrary_namespaces_and_urls() {
    assert_eq!(
        IconReference::parse("custom-pack:star").unwrap().as_str(),
        "custom-pack:star"
    );
    assert_eq!(
        IconReference::parse("https://assets.example/icon.svg")
            .unwrap()
            .as_str(),
        "https://assets.example/icon.svg"
    );
}

#[test]
fn icon_reference_rejects_only_empty_values() {
    assert_eq!(
        IconReference::parse("  "),
        Err(IconReferenceError::Empty {
            value: "  ".to_owned()
        })
    );
}
