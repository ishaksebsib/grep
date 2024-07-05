use std::env;
use std::process;

use grep::run;
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parshing arguments : {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application Error ;( {}", e);
        process::exit(1)
    }
}
