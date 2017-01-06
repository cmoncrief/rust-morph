use converter::{convert_case, CapitalizeType};

/// Convert a string to a title cased string
///
/// # Examples
///
/// ```
/// assert_eq!("Lorem Ipsum Dolor", string_morph::to_title_case("lorem-ipsum-dolor"))
/// ```

pub fn to_title_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWord, true, ' ')
}
