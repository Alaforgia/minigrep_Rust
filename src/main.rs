use std::env;
fn main() {
// The program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
 let args: Vec<String> = env::args().collect();
 
 let query = &args[1];
 let filename = &args[2];

 println!("Searching for {}", query);
 println!("In file {}", filename)
}
