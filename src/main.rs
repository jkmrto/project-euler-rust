mod position;
mod exercise_11;
mod read_csv;

fn main() {
    let filename = "./matrix.csv";
    let matrix = read_csv::read_matrix_from_file(&filename);
    // matrix_printer(&matrix)
}
