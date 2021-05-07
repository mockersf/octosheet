use octosheet::*;

#[test]
fn can_parse_file_ode_to_joy() {
    assert!(dbg!(parse_file("sheets/Ode_to_joy_Piano_for_kids.xml")).is_ok());
}

#[test]
fn can_parse_file_jurassic_park() {
    assert!(dbg!(parse_file("sheets/John_Williams_-_Jurassic_Park.xml")).is_ok());
}
