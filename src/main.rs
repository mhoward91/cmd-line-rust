// run program with cargo run -- {string to search} poem.txt > output.txt
// -- means program arguments rather than arguments to cargo
// > means direct output to txt file instead of terminal
// eprintln! macro being used to direct these messages to the standard error stream, 
// meaning use of > will only send standard output to the txt file
// can use environment variable IGNORE_CASE to do a case insensitive search
// set by calling IGNORE_CASE=1 ahead of the cargo run command

use std::{env, process};
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}