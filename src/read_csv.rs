use std::env;
use std::fs;

fn main() {
    let filename = "./matrix.csv";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");
    let temp_vec: Vec<&str> = split.collect();

    println!("Printing:");
    for row in &temp_vec{
        let split_row: Vec<u32>  = row // string
            .split(" ") // Iter::vec<&str>
            .map(|s| s.parse::<u32>().unwrap()) // Iter::vec<u32> 
            .collect(); // vec <u32>
            
         for eo in &split_row{
             println!("{}", eo);
         }
    }
}