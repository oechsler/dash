use super::{Color, ColorError, ThemeError};

#[test]
fn color_accepts_three_and_six_digit_hex_values() {
    assert_eq!(Color::parse("#fff").unwrap().as_str(), "#fff");
    assert_eq!(Color::parse("#cba6f7").unwrap().as_str(), "#cba6f7");
}

#[test]
fn color_rejects_non_hex_values_with_context() {
    assert_eq!(
        Color::parse("red"),
        Err(ThemeError::Color(ColorError::InvalidHex {
            value: "red".to_owned()
        }))
    );
}
