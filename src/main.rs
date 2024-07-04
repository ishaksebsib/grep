use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let mut f = File::open(file_name).expect("File not Found ;(");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading this file ;(");

    println!("\n--- File Content ---\n\n{}",contents);


}
