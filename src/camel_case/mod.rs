use converter::{convert_case, CapitalizeType};

/// Convert a string to a camel cased string
///
/// # Examples
///
/// ```
/// assert_eq!("loremIpsumDolor", string_morph::to_camel_case("lorem_ipsum_dolor"))
/// ```

pub fn to_camel_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWordExceptFirst, false, ' ')
}
