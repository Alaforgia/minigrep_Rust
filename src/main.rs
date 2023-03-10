use std::env;
use std::fs; // handles files
use std::process;
use std::error::Error;
fn main() {
// The program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
 let args: Vec<String> = env::args().collect();

 let config = Config::new(&args).unwrap_or_else(|err| {
     println!("Problem parsing arguments: {}", err);
     process::exit(1);
    });
 

 println!("Searching for {}", config.query);
 println!("In file {}", config.filename);

 if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
    }

//  run(config);
}
 // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents.

 fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    Ok(())
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