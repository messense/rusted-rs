extern crate syntect;
extern crate pulldown_cmark;

use std::io::{self, Read};

use pulldown_cmark::Parser;

mod renderer;

use renderer::render;

fn main() {
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(..) => {
            highlight(&input);
        },
        Err(error) => println!("error: {}", error),
    }
}


fn highlight(input: &str) {
    let mut rendered = String::new();
    let parser = Parser::new(input);
    render(&mut rendered, parser);
    print!("{}", &rendered);
}
