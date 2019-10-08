use std::env;
use std::fs;

fn main() {
    let filename = "./matrix.csv";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");
    let temp_vec: Vec<&str> = split.collect();

    let mut vec = Vec::new();

    println!("Printing:");
    for element in &vec{
        println!("{}", element);
    }
}