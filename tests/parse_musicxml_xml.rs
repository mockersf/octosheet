use octosheet::*;

use test_generator::test_resources;

#[test_resources("sheets/*.xml")]
fn can_parse_file(file: &str) {
    assert!(dbg!(parse_file(&file)).is_ok());
}
