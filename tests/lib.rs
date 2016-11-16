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
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps("loremIpsumDolor"))
}

#[test]
fn test_snake_caps_from_camel_acronyms() {
    assert_eq!("NASA_AND_THE_JPL_HAVE_A_ROCKET", morph::to_snake_caps("NASAAndTheJPLHaveARocket"))
}

#[test]
fn test_snake_caps_from_sentence() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps("Lorem ipsum dolor"))
}

#[test]
fn test_snake_caps_from_dashed() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps("lorem-ipsum-dolor"))
}

#[test]
fn test_snake_caps_from_dotted() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps("lorem.ipsum.dolor"))
}

#[test]
fn test_snake_caps_from_snake() {
    assert_eq!("LOREM_IPSUM_DOLOR", morph::to_snake_caps("lorem_ipsum_dolor"))
}

#[test]
fn test_snake_caps_from_mixed() {
    assert_eq!("LOREM_IPSUM_DOLOR_SIT_AMET", morph::to_snake_caps("LOREM.ipsum-DolorSit_amet"))
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

///////////////////////////////////////////////////////////////////////////
// Dashed / Kebab
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
