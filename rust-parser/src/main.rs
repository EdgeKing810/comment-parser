use comments::parser;
use data::obtain_data;

mod comments;
mod data;

fn main() {
    match parser(obtain_data()) {
        Ok(()) => println!("\nSuccessfully Parsed JSON!"),
        _ => println!("\nFailed Parsing JSON"),
    };
}
