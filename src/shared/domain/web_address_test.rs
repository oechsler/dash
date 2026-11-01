use super::{WebAddress, WebAddressError};

#[test]
fn web_address_requires_an_absolute_address() {
    assert!(WebAddress::try_from("https://example.com").is_ok());
    assert!(matches!(
        WebAddress::try_from("example.com"),
        Err(WebAddressError::Malformed { .. })
    ));
}
