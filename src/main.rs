use std::env;
use std::fs; // handles files
use std::process;
fn main() {
// The program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
 let args: Vec<String> = env::args().collect();

 let config = Config::new(&args).unwrap_or_else(|err| {
     println!("Problem parsing arguments: {}", err);
     process::exit(1);
    });
 

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

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}