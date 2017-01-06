//! # string_morph
//!
//! string_morph is a fast, accurate library for string case transformations. It exposes
//! both functions as well as traits for `String` and `str`.
//!
//! ```
//! use string_morph::Morph;
//!
//! // Camel case
//! assert_eq!("loremIpsumDolor", string_morph::to_camel_case("lorem_ipsum_dolor"));
//! assert_eq!("loremIpsumDolor", "lorem_ipsum_dolor".to_camel_case());
//!
//! // Pascal case
//! assert_eq!("LoremIpsumDolor", string_morph::to_pascal_case("lorem_ipsum_dolor"));
//! assert_eq!("LoremIpsumDolor", "lorem_ipsum_dolor".to_pascal_case());
//!
//! // Kebab case
//! assert_eq!("lorem-ipsum-dolor", string_morph::to_kebab_case("lorem_ipsum_dolor"));
//! assert_eq!("lorem-ipsum-dolor", "lorem_ipsum_dolor".to_kebab_case());
//!
//! // Sentence case
//! assert_eq!("Lorem ipsum dolor", string_morph::to_sentence_case("lorem_ipsum_dolor"));
//! assert_eq!("Lorem ipsum dolor", "lorem_ipsum_dolor".to_sentence_case());
//!
//! // Snake case
//! assert_eq!("lorem_ipsum_dolor", string_morph::to_snake_case("Lorem ipsum dolor"));
//! assert_eq!("lorem_ipsum_dolor", "Lorem ipsum dolor".to_snake_case());
//!
//! // Upper snake case
//! assert_eq!("LOREM_IPSUM_DOLOR", string_morph::to_snake_caps_case("Lorem ipsum dolor"));
//! assert_eq!("LOREM_IPSUM_DOLOR", "Lorem ipsum dolor".to_snake_caps_case());
//!
//! // Title case
//! assert_eq!("Lorem Ipsum Dolor", string_morph::to_title_case("lorem-ipsum-dolor"));
//! assert_eq!("Lorem Ipsum Dolor", "lorem-ipsum-dolor".to_title_case());
//!
//! // Upper first
//! assert_eq!("Test", string_morph::to_upper_first("test"));
//! assert_eq!("Test", "test".to_upper_first());
//!
//! ```

mod converter;
pub mod snake_case;
pub mod pascal_case;
pub mod kebab_case;
pub mod camel_case;
pub mod upper_first;
pub mod sentence_case;
pub mod title_case;

pub use self::snake_case::{to_snake_case, to_snake_caps_case};
pub use self::kebab_case::{to_kebab_case};
pub use self::camel_case::{to_camel_case};
pub use self::pascal_case::{to_pascal_case};
pub use self::upper_first::{to_upper_first};
pub use self::sentence_case::{to_sentence_case};
pub use self::title_case::{to_title_case};

pub trait Morph {
    fn to_snake_case(&self) -> String;
    fn to_snake_caps_case(&self) -> String;
    fn to_kebab_case(&self) -> String;
    fn to_camel_case(&self) -> String;
    fn to_pascal_case(&self) -> String;
    fn to_sentence_case(&self) -> String;
    fn to_title_case(&self) -> String;
    fn to_upper_first(&self) -> String;
}

impl<'a> Morph for String {
    fn to_snake_case(&self) -> String { to_snake_case(&self)}
    fn to_snake_caps_case(&self) -> String { to_snake_caps_case(&self)}
    fn to_kebab_case(&self) -> String { to_kebab_case(&self)}
    fn to_camel_case(&self) -> String { to_camel_case(&self)}
    fn to_pascal_case(&self) -> String { to_pascal_case(&self)}
    fn to_sentence_case(&self) -> String { to_sentence_case(&self)}
    fn to_title_case(&self) -> String { to_title_case(&self)}
    fn to_upper_first(&self) -> String { to_upper_first(&self)}
}

impl<'a> Morph for str {
    fn to_snake_case(&self) -> String { to_snake_case(&self)}
    fn to_snake_caps_case(&self) -> String { to_snake_caps_case(&self)}
    fn to_kebab_case(&self) -> String { to_kebab_case(&self)}
    fn to_camel_case(&self) -> String { to_camel_case(&self)}
    fn to_pascal_case(&self) -> String { to_pascal_case(&self)}
    fn to_sentence_case(&self) -> String { to_sentence_case(&self)}
    fn to_title_case(&self) -> String { to_title_case(&self)}
    fn to_upper_first(&self) -> String { to_upper_first(&self)}
}
