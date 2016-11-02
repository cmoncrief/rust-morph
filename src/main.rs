extern crate morph;

// use morph::snake_case;

fn main() {

    let test_string = "NS Object Definition";

    let output: String = morph::to_upper_first(test_string);
    println!("Upper first: {}", output);

    let output: String = morph::to_snake_case(test_string);
    println!("Snake case to: {}", output);

    let output: String = morph::to_snake_caps(test_string);
    println!("Snake caps: {}", output);

    let output: String = morph::to_kebab(test_string);
    println!("Kebab: {}", output);

    let output: String = morph::to_camel(test_string);
    println!("Camel: {}", output);

    let output: String = morph::to_upper_camel(test_string);
    println!("Upper Camel: {}", output);

    let output: String = morph::to_sentence(test_string);
    println!("Sentence: {}", output);

    let output: String = morph::to_title(test_string);
    println!("Title: {}", output);
}
