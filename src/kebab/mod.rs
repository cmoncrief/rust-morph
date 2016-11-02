use converter::{convert_case, CapitalizeType};

pub fn to_kebab(input: &str) -> String {
    convert_case(input.to_string(), CapitalizeType::AllLowercase, true, '-')
}
