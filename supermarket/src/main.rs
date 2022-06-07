mod user;

use std::env;
use std::fs;

pub fn flush() {
    std::process::Command::new("clear").status().unwrap();
}

fn print_action() {
    flush();
    println!("■■■■■■■■■■■■■ Welcome to *NFT Chain* shop!■■■■■■■■■■■■■■■■■■■■■");
}

// The program takes as input the filename
fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let catalog = json::parse(&contents.to_string()).unwrap();

    print_action();

    println!("{}", catalog);
    loop{

    }
}
