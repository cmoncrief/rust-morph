use converter::{convert_case, CapitalizeType};

pub fn to_sentence_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::FirstLetterOfString, true, ' ')
}
