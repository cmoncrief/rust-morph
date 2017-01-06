#![feature(test)]
extern crate morph;
extern crate test;
use test::Bencher;

#[bench]
fn bench_snake_case(b: &mut Bencher) {
    b.iter(|| string_morph::to_snake_case("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_snake_caps(b: &mut Bencher) {
    b.iter(|| string_morph::to_snake_caps_case("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_dashed(b: &mut Bencher) {
    b.iter(|| string_morph::to_kebab_case("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_first_case(b: &mut Bencher) {
    b.iter(|| string_morph::to_upper_first("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_camel(b: &mut Bencher) {
    b.iter(|| string_morph::to_camel_case("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_pascal(b: &mut Bencher) {
    b.iter(|| string_morph::to_pascal_case("NASA and the JPL have a rocket"))
}

#[bench]
fn bench_human(b: &mut Bencher) {
    b.iter(|| string_morph::to_sentence_case("NASAAndTheJPLHaveARocket"))
}

#[bench]
fn bench_title(b: &mut Bencher) {
    b.iter(|| string_morph::to_title_case("NASAAndTheJPLHaveARocket"))
}
