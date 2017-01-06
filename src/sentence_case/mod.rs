use converter::{convert_case, CapitalizeType};

/// Convert a string to a human sentence cased string
///
/// # Examples
///
/// ```
/// assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("lorem_ipsum_dolor"))
/// ```

pub fn to_sentence_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfString, true, ' ')
}
