use converter::{convert_case, CapitalizeType};

pub fn to_snake_case(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllLowercase, true, '_')
}

pub fn to_snake_caps(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllUppercase, true, '_')
}
