use converter::{convert_case, CapitalizeType};

pub fn to_title(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWord, true, ' ')
}
