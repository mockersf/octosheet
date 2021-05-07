use octosheet::*;
use std::env;

fn main() {
    match env::args().nth(1) {
        None => println!("please provide a MusicXML file"),
        Some(file) => println!("{:?}", dbg!(parse_file(&file))),
    }
}
