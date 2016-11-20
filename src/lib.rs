#![feature(test)]
extern crate test;

mod converter;
pub mod snake_case;
pub mod kebab;
pub mod camel;
pub mod first_case;
pub mod sentence;
pub mod title;

pub use self::snake_case::{to_snake_case, to_snake_caps_case};
pub use self::kebab::{to_kebab_case};
pub use self::camel::{to_camel_case, to_upper_camel_case};
pub use self::first_case::{to_upper_first};
pub use self::sentence::{to_sentence_case};
pub use self::title::{to_title_case};

pub trait Morph {
    fn to_snake_case(&self) -> String;
    fn to_snake_caps_case(&self) -> String;
    fn to_kebab_case(&self) -> String;
    fn to_camel_case(&self) -> String;
    fn to_upper_camel_case(&self) -> String;
    fn to_sentence_case(&self) -> String;
    fn to_title_case(&self) -> String;
}

impl<'a> Morph for String {
    fn to_snake_case(&self) -> String { to_snake_case(&self)}
    fn to_snake_caps_case(&self) -> String { to_snake_caps_case(&self)}
    fn to_kebab_case(&self) -> String { to_kebab_case(&self)}
    fn to_camel_case(&self) -> String { to_camel_case(&self)}
    fn to_upper_camel_case(&self) -> String { to_upper_camel_case(&self)}
    fn to_sentence_case(&self) -> String { to_sentence_case(&self)}
    fn to_title_case(&self) -> String { to_title_case(&self)}
}

impl<'a> Morph for str {
    fn to_snake_case(&self) -> String { to_snake_case(&self)}
    fn to_snake_caps_case(&self) -> String { to_snake_caps_case(&self)}
    fn to_kebab_case(&self) -> String { to_kebab_case(&self)}
    fn to_camel_case(&self) -> String { to_camel_case(&self)}
    fn to_upper_camel_case(&self) -> String { to_upper_camel_case(&self)}
    fn to_sentence_case(&self) -> String { to_sentence_case(&self)}
    fn to_title_case(&self) -> String { to_title_case(&self)}
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_snake_case(b: &mut Bencher) {
        b.iter(|| to_snake_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_snake_caps(b: &mut Bencher) {
        b.iter(|| to_snake_caps_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_dashed(b: &mut Bencher) {
        b.iter(|| to_kebab_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_first_case(b: &mut Bencher) {
        b.iter(|| to_upper_first("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_camel(b: &mut Bencher) {
        b.iter(|| to_camel_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_upper_camel(b: &mut Bencher) {
        b.iter(|| to_upper_camel_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_human(b: &mut Bencher) {
        b.iter(|| to_sentence_case("NASAAndTheJPLHaveARocket"))
    }

    #[bench]
    fn bench_title(b: &mut Bencher) {
        b.iter(|| to_title_case("NASAAndTheJPLHaveARocket"))
    }

}
