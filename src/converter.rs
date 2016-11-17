use std::ascii::*;

pub enum CapitalizeType {
    FirstLetterOfString,
    FirstLetterOfWord,
    FirstLetterOfWordExceptFirst,
    AllLowercase,
    AllUppercase,
}

const SEPARATORS: [char; 4] = ['-', '_', ' ', '.'];

pub fn convert_case(input: String, cap_type: CapitalizeType, use_separator: bool, separator: char) -> String {

    let char_ref: Vec<char> = input.chars().collect();
    let len = input.len();
    let (mut first_char, mut last_char) = (true, false);
    let (mut prev_sep, mut next_sep, mut cur_sep) = (false, false, false);
    let (mut prev_upper, mut next_upper, mut cur_upper) = (false, false, false);
    let (mut is_boundary, mut is_pre_boundary, mut is_post_boundary) = (false, false, false);

    let is_all_upper = !input.chars().any(|c| c.is_lowercase());

    if len <= 1 {
        return input.clone();
    }

    input.chars().enumerate().fold("".to_string(), |mut output, index_char| {

        cur_upper = index_char.1.is_uppercase();
        cur_sep = SEPARATORS.contains(&index_char.1);

        if (index_char.0 + 1) < len {
            next_upper = char_ref[index_char.0 + 1].is_uppercase();
            next_sep = SEPARATORS.contains(&char_ref[index_char.0 + 1])
        } else {
            next_upper = false;
            next_sep = false;
        }

        // println!("current: {} cur_upper: {} next_upper: {} prev_upper: {} last_char: {} index: {}",
        //          index_char.1, cur_upper, next_upper, prev_upper, last_char, index_char.0);

        if last_char {
            is_post_boundary = false;
            is_pre_boundary = false;
            is_boundary = false;
        } else if cur_sep {
            is_boundary = true;
        } else if !cur_upper && next_upper {
            is_post_boundary = true;
        } else if prev_upper && cur_upper && !next_upper && !next_sep {
            is_pre_boundary = true;
        } else {
            is_post_boundary = false;
            is_pre_boundary = false;
            is_boundary = false;
        }

        let new_char = match cap_type {
            CapitalizeType::AllLowercase => index_char.1.to_ascii_lowercase(),
            CapitalizeType::AllUppercase => index_char.1.to_ascii_uppercase(),

            CapitalizeType::FirstLetterOfString => {
                if first_char {
                    index_char.1.to_ascii_uppercase()
                } else if is_all_upper {
                    index_char.1.to_ascii_lowercase()
                } else if cur_upper && (prev_upper || next_upper) {
                    index_char.1
                } else {
                    index_char.1.to_ascii_lowercase()
                }
            },

            CapitalizeType::FirstLetterOfWord => {
                if first_char || prev_sep || is_pre_boundary {
                    index_char.1.to_ascii_uppercase()
                } else if is_all_upper {
                    index_char.1.to_ascii_lowercase()
                } else if cur_upper && (prev_upper || next_upper) {
                    index_char.1
                } else {
                    index_char.1.to_ascii_lowercase()
                }
            },

            CapitalizeType::FirstLetterOfWordExceptFirst => {
                if first_char && cur_upper && next_upper && !is_all_upper {
                    index_char.1
                } else if first_char {
                    index_char.1.to_ascii_lowercase()
                } else if prev_sep || is_pre_boundary {
                    index_char.1.to_ascii_uppercase()
                } else if is_all_upper {
                    index_char.1.to_ascii_lowercase()
                } else if cur_upper && (prev_upper || next_upper) {
                    index_char.1
                } else {
                    index_char.1.to_ascii_lowercase()
                }
            },
        };

        if is_boundary {
            if use_separator {output.push(separator)};
        } else if is_pre_boundary {
            if use_separator {output.push(separator)};
            output.push(new_char);
        } else if is_post_boundary {
            output.push(new_char);
            if use_separator {output.push(separator)};
        } else {
            output.push(new_char);
        }

        first_char = false;
        last_char = if index_char.0 == (len - 2) {true} else {false};
        prev_sep = if cur_sep || is_post_boundary {true} else {false};
        prev_upper = cur_upper;
        output
    })

}
