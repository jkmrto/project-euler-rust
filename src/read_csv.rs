use std::fmt;
use std::fs;

fn main() {
    let filename = "./matrix.csv";
    let matrix = read_matrix_from_file(&filename);
    println!("{}", matrix[0][4])
}
 
fn read_matrix_from_file(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let rows: Vec<&str>  = contents.split("\n").collect();
    let mut splitted_rows: Vec<Vec<u32>> = Vec::new();
    for row in rows {
        println!("{}", row);
        let splitted_row = row // string
            .split(", ") // Iter::vec<&str>
            .map(|s| s.parse::<u32>().unwrap()) // Iter::vec<u32>
            .collect(); // vec <u32>
        splitted_rows.push(splitted_row)
    }
    splitted_rows
}
