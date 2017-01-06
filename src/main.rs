extern crate morph;

use morph::Morph;

fn main() {

    let test_string = "NASAAndTheJPLHaveARocket";

    println!("Snake case string: {}",test_string.to_snake_case());
    let output: String = morph::to_upper_first(test_string);
    println!("Upper first: {}", output);

    let output: String = morph::to_snake_case(test_string);
    println!("Snake case to: {}", output);

    let output: String = morph::to_snake_caps_case(test_string);
    println!("Snake caps: {}", output);

    let output: String = morph::to_kebab_case(test_string);
    println!("Kebab: {}", output);

    let output: String = morph::to_camel_case(test_string);
    println!("Camel: {}", output);

    let output: String = morph::to_pascal_case(test_string);
    println!("Pascal: {}", output);

    let output: String = morph::to_sentence_case(test_string);
    println!("Sentence: {}", output);

    let output: String = morph::to_title_case(test_string);
    println!("Title: {}", output);
}
