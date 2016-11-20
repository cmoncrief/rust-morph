use converter::{convert_case, CapitalizeType};

pub fn to_title_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWord, true, ' ')
}
