# Morph

Morph is a Rust library for string case transformations. It has an emphasis on accuracy and speed, in that order. The case conversions are available as functions as well as traits on String types.

# Examples

      use morph::Morph;

      // Camel case
      assert_eq!("loremIpsumDolor", morph::to_camel_case("lorem_ipsum_dolor"));
      assert_eq!("loremIpsumDolor", "lorem_ipsum_dolor".to_camel_case());

      // Pascal case
      assert_eq!("LoremIpsumDolor", morph::to_pascal_case("lorem_ipsum_dolor"));
      assert_eq!("LoremIpsumDolor", "lorem_ipsum_dolor".to_pascal_case());

      // Kebab case
      assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("lorem_ipsum_dolor"));
      assert_eq!("lorem-ipsum-dolor", "lorem_ipsum_dolor".to_kebab_case());

      // Sentence case
      assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("lorem_ipsum_dolor"));
      assert_eq!("Lorem ipsum dolor", "lorem_ipsum_dolor".to_sentence_case());

      // Snake case
      assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("Lorem ipsum dolor"));
      assert_eq!("lorem_ipsum_dolor", "Lorem ipsum dolor".to_snake_case());

      // Upper snake case
      assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("Lorem ipsum dolor"));
      assert_eq!("LOREM_IPSUM_DOLOR", "Lorem ipsum dolor".to_snake_caps_case());

      // Title case
      assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("lorem-ipsum-dolor"));
      assert_eq!("Lorem Ipsum Dolor", "lorem-ipsum-dolor".to_title_case());

      // Upper first
      assert_eq!("Test", morph::to_upper_first("test"));
      assert_eq!("Test", "test".to_upper_first());


  # Benchmarks

    running 8 tests
    test bench_camel      ... bench:         538 ns/iter (+/- 28)
    test bench_dashed     ... bench:         517 ns/iter (+/- 38)
    test bench_first_case ... bench:          68 ns/iter (+/- 1)
    test bench_human      ... bench:         578 ns/iter (+/- 11)
    test bench_pascal     ... bench:         611 ns/iter (+/- 22)
    test bench_snake_caps ... bench:         529 ns/iter (+/- 23)
    test bench_snake_case ... bench:         510 ns/iter (+/- 52)
    test bench_title      ... bench:         566 ns/iter (+/- 58)
