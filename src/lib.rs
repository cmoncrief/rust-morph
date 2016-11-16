#![feature(test)]
extern crate test;

mod converter;
pub mod snake_case;
pub mod kebab;
pub mod camel;
pub mod first_case;
pub mod sentence;
pub mod title;

pub use self::snake_case::{to_snake_case, to_snake_caps};
pub use self::kebab::{to_kebab_case};
pub use self::camel::{to_camel_case, to_upper_camel_case};
pub use self::first_case::{to_upper_first};
pub use self::sentence::{to_sentence};
pub use self::title::{to_title};

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    //  #[test]
    //     fn it_works() {
    //     }

    #[bench]
    fn bench_snake_case(b: &mut Bencher) {
        b.iter(|| to_snake_case("this is a test string"))
    }

    #[bench]
    fn bench_snake_caps(b: &mut Bencher) {
        b.iter(|| to_snake_caps("this is a test string"))
    }

    #[bench]
    fn bench_dashed(b: &mut Bencher) {
        b.iter(|| to_kebab_case("this is a test string"))
    }

    #[bench]
    fn bench_first_case(b: &mut Bencher) {
        b.iter(|| to_upper_first("this is a test string"))
    }

    #[bench]
    fn bench_camel(b: &mut Bencher) {
        b.iter(|| to_camel_case("this is a test string"))
    }

    #[bench]
    fn bench_upper_camel(b: &mut Bencher) {
        b.iter(|| to_upper_camel_case("this is a test string"))
    }

    #[bench]
    fn bench_human(b: &mut Bencher) {
        b.iter(|| to_sentence("this is a test string"))
    }

    #[bench]
    fn bench_title(b: &mut Bencher) {
        b.iter(|| to_title("this is a test string"))
    }

}
