use converter::{convert_case, CapitalizeType};

/// Convert a string to a pascal cased string
///
/// # Examples
///
/// ```
/// assert_eq!("LoremIpsumDolor", morph::to_pascal_case("lorem_ipsum_dolor"))
/// ```

pub fn to_pascal_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWord, false, ' ')
}
