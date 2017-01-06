use converter::{convert_case, CapitalizeType};

/// Convert a string to a kebab cased string
///
/// # Examples
///
/// ```
/// assert_eq!("lorem-ipsum-dolor", string_morph::to_kebab_case("lorem_ipsum_dolor"))
/// ```

pub fn to_kebab_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllLowercase, true, '-')
}
