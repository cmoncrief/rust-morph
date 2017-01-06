pub fn to_upper_first(input: &str) -> String {
    let mut c = input.chars();

    match c.next() {
        None => String::new(),
        Some(x) => x.to_uppercase().collect::<String>() + c.as_str()
    }
}

pub fn to_lower_first(input: &str) -> String {
    let mut c = input.chars();

    match c.next() {
        None => String::new(),
        Some(x) => x.to_lowercase().collect::<String>() + c.as_str()
    }
}
