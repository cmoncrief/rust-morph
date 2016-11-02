use converter::{convert_case, CapitalizeType};

pub fn to_camel(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWordExceptFirst, false, ' ')
}

pub fn to_upper_camel(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfWord, false, ' ')
}
