use converter::{convert_case, CapitalizeType};

/// Convert a string to a snake cased string
///
/// # Examples
///
/// ```
/// assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("Lorem ipsum dolor"))
/// ```

pub fn to_snake_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllLowercase, true, '_')
}

/// Convert a string to an uppercase snake cased string
///
/// # Examples
///
/// ```
/// assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("Lorem ipsum dolor"))
/// ```

pub fn to_snake_caps_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllUppercase, true, '_')
}
