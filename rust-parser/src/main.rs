use comments::parser;

mod comments;

fn main() {
    println!("Hello, world!");

    match parser() {
        Ok(()) => println!("\nSuccessfully Parsed JSON!"),
        _ => println!("\nFailed Parsing JSON"),
    };
}
