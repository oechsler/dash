use super::{WebAddress, WebAddressError};

#[test]
fn web_address_requires_an_absolute_address() {
    assert!(WebAddress::parse("https://example.com").is_ok());
    assert!(matches!(
        WebAddress::parse("example.com"),
        Err(WebAddressError::Malformed { .. })
    ));
}
