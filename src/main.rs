use std::env;
use std::fs; // handles files
fn main() {
// The program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
 let args: Vec<String> = env::args().collect();

 let config = parse_config(&args);
 

 println!("Searching for {}", config.query);
 println!("In file {}", config.filename);

 // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents.
 let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

 println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}