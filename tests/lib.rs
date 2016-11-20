extern crate morph;

///////////////////////////////////////////////////////////////////////////
// Snake case
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_snake_from_camel() {
    assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("loremIpsumDolor"))
}

#[test]
fn test_snake_from_camel_acronyms() {
    assert_eq!("nasa_and_the_jpl", morph::to_snake_case("NASAAndTheJPL"))
}

#[test]
fn test_snake_from_sentence() {
    assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("Lorem ipsum dolor"))
}

#[test]
fn test_snake_from_dashed() {
    assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("lorem-ipsum-dolor"))
}

#[test]
fn test_snake_from_dotted() {
    assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("lorem.ipsum.dolor"))
}

#[test]
fn test_snake_from_snake() {
    assert_eq!("lorem_ipsum_dolor", morph::to_snake_case("lorem_ipsum_dolor"))
}

#[test]
fn test_snake_from_mixed() {
    assert_eq!("lorem_ipsum_dolor_sit_amet", morph::to_snake_case("LOREM.ipsum-DolorSit_amet"))
}

///////////////////////////////////////////////////////////////////////////
// Snake caps
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_snake_caps_from_camel() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("loremIpsumDolor"))
}

#[test]
fn test_snake_caps_from_camel_acronyms() {
    assert_eq!("NASA_AND_THE_JPL_HAVE_A_ROCKET", morph::to_snake_caps_case("NASAAndTheJPLHaveARocket"))
}

#[test]
fn test_snake_caps_from_sentence() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("Lorem ipsum dolor"))
}

#[test]
fn test_snake_caps_from_dashed() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("lorem-ipsum-dolor"))
}

#[test]
fn test_snake_caps_from_dotted() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("lorem.ipsum.dolor"))
}

#[test]
fn test_snake_caps_from_snake() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps_case("lorem_ipsum_dolor"))
}

#[test]
fn test_snake_caps_from_mixed() {
    assert_eq!("LOREM_IPSUM_DOLOR_SIT_AMET", morph::to_snake_caps_case("LOREM.ipsum-DolorSit_amet"))
}

///////////////////////////////////////////////////////////////////////////
// Dashed / Kebab
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_kebab_case_from_camel() {
    assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("loremIpsumDolor"))
}

#[test]
fn test_kebab_case_from_camel_acronyms() {
    assert_eq!("nasa-and-the-jpl-have-a-rocket", morph::to_kebab_case("NASAAndTheJPLHaveARocket"))
}

#[test]
fn test_kebab_case_from_sentence() {
    assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("Lorem ipsum dolor"))
}

#[test]
fn test_kebab_case_from_snake() {
    assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("lorem_ipsum_dolor"))
}

#[test]
fn test_kebab_case_from_dotted() {
    assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("lorem.ipsum.dolor"))
}

#[test]
fn test_kebab_case_from_mixed() {
    assert_eq!("lorem-ipsum-dolor-sit-amet", morph::to_kebab_case("LOREM.ipsum-DolorSit_amet"))
}

#[test]
fn test_kebab_case_from_kebab() {
    assert_eq!("lorem-ipsum-dolor", morph::to_kebab_case("lorem-ipsum-dolor"))
}

///////////////////////////////////////////////////////////////////////////
// Camel
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_camel_case_from_snake() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("lorem_ipsum_dolor"))
}

#[test]
fn test_camel_case_from_snake_caps() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("LOREM_IPSUM_DOLOR"))
}

#[test]
fn test_camel_case_from_snake_acronyms() {
    assert_eq!("NASAAndTheJPLHaveARocket", morph::to_camel_case("NASA_and_the_JPL_have_a_rocket"))
}

#[test]
fn test_camel_case_from_sentence() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("Lorem ipsum dolor"))
}
#[test]
fn test_camel_case_from_dotted() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("lorem.ipsum.dolor"))
}

#[test]
fn test_camel_case_from_dashed() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("lorem-ipsum-dolor"))
}

#[test]
fn test_camel_case_from_mixed() {
    assert_eq!("loremIpsumDolorSitAmet", morph::to_camel_case("Lorem.ipsum-DolorSit_amet"))
}

#[test]
fn test_camel_case_from_camel() {
    assert_eq!("loremIpsumDolor", morph::to_camel_case("loremIpsumDolor"))
}

///////////////////////////////////////////////////////////////////////////
// Upper camel
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_upper_camel_case_from_snake() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("lorem_ipsum_dolor"))
}

#[test]
fn test_upper_camel_case_from_snake_caps() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("LOREM_IPSUM_DOLOR"))
}

#[test]
fn test_upper_camel_case_from_snake_acronyms() {
    assert_eq!("NASAAndTheJPLHaveARocket", morph::to_upper_camel_case("NASA_and_the_JPL_have_a_rocket"))
}

#[test]
fn test_upper_camel_case_from_sentence() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("Lorem ipsum dolor"))
}
#[test]
fn test_upper_camel_case_from_dotted() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("lorem.ipsum.dolor"))
}

#[test]
fn test_upper_camel_case_from_dashed() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("lorem-ipsum-dolor"))
}

#[test]
fn test_upper_camel_case_from_mixed() {
    assert_eq!("LoremIpsumDolorSitAmet", morph::to_upper_camel_case("Lorem.ipsum-DolorSit_amet"))
}

#[test]
fn test_upper_camel_case_from_camel() {
    assert_eq!("LoremIpsumDolor", morph::to_upper_camel_case("loremIpsumDolor"))
}

///////////////////////////////////////////////////////////////////////////
// Human (Sentence)
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_sentence_case_from_snake() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("lorem_ipsum_dolor"))
}

#[test]
fn test_sentence_case_from_snake_caps() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("LOREM_IPSUM_DOLOR"))
}

#[test]
fn test_sentence_case_from_snake_acronyms() {
    assert_eq!("NASA and the JPL have a rocket", morph::to_sentence_case("NASA_and_the_JPL_have_a_rocket"))
}

#[test]
fn test_sentence_case_from_camel_acronyms() {
    assert_eq!("NASA and the JPL have a rocket", morph::to_sentence_case("NASAAndTheJPLHaveARocket"))
}

#[test]
fn test_sentence_case_from_dotted() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("lorem.ipsum.dolor"))
}

#[test]
fn test_sentence_case_from_dashed() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("lorem-ipsum-dolor"))
}

#[test]
fn test_sentence_case_from_mixed() {
    assert_eq!("Lorem ipsum dolor sit amet", morph::to_sentence_case("Lorem.ipsum-DolorSit_amet"))
}

#[test]
fn test_sentence_case_from_camel() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("loremIpsumDolor"))
}

#[test]
fn test_sentence_case_from_sentence() {
    assert_eq!("Lorem ipsum dolor", morph::to_sentence_case("Lorem ipsum dolor"))
}

#[test]
fn test_sentence_case_single_letter_words() {
    assert_eq!("This is a camel case word", morph::to_sentence_case("thisIsACamelCaseWord"))
}

///////////////////////////////////////////////////////////////////////////
// Title
///////////////////////////////////////////////////////////////////////////

#[test]
fn test_title_case_from_snake() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("lorem_ipsum_dolor"))
}

#[test]
fn test_title_case_from_snake_caps() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("LOREM_IPSUM_DOLOR"))
}

#[test]
fn test_title_case_from_snake_acronyms() {
    assert_eq!("NASA And The JPL Have A Rocket", morph::to_title_case("NASA_and_the_JPL_have_a_rocket"))
}

#[test]
fn test_title_case_from_camel_acronyms() {
    assert_eq!("NASA And The JPL Have A Rocket", morph::to_title_case("NASAAndTheJPLHaveARocket"))
}

#[test]
fn test_title_case_from_dotted() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("lorem.ipsum.dolor"))
}

#[test]
fn test_title_case_from_dashed() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("lorem-ipsum-dolor"))
}

#[test]
fn test_title_case_from_mixed() {
    assert_eq!("Lorem Ipsum Dolor Sit Amet", morph::to_title_case("Lorem.ipsum-DolorSit_amet"))
}

#[test]
fn test_title_case_from_camel() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("loremIpsumDolor"))
}

#[test]
fn test_title_case_from_sentence() {
    assert_eq!("Lorem Ipsum Dolor", morph::to_title_case("Lorem ipsum dolor"))
}

#[test]
fn test_title_case_single_letter_words() {
    assert_eq!("This Is A Camel Case Word", morph::to_title_case("thisIsACamelCaseWord"))
}
