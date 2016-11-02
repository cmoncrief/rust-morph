use converter::{convert_case, CapitalizeType};

pub fn to_sentence(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfString, true, ' ')
}
