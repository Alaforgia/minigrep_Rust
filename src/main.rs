use std::env;
use std::fs; // handles files
fn main() {
// The program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
 let args: Vec<String> = env::args().collect();
 
 let query = &args[1];
 let filename = &args[2];

 println!("Searching for {}", query);
 println!("In file {}", filename);

 // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents.
 let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

 println!("With text: \n{}", contents);
}
