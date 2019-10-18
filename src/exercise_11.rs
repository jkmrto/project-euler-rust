use std::fmt;

struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn are_valid_index(positions: &[Position; 3], limit: &Position) -> bool {
    (limit.x > positions[2].x) && (limit.y > positions[2].y)
}

fn update_positions(positions: &mut [Position; 3], shift: &Position) {
    for position in positions {
        position.x += shift.x;
        position.y += shift.y;
    }
}

// fn print_positions(positions: &[Position; 3]) {
//     println!("{}, {}, {}", positions[0], positions[1], positions[2])
// }

fn calculate_product(matrix: &Vec<Vec<u32>>, positions: &[Position; 3]) -> u32 {
    let mut acc = 1;
    for position in positions {
        acc *= matrix[position.x][position.y]
    }
    acc
}

fn matrix_printer(matrix: &Vec<Vec<u32>>) {
    let mut max_product: u32 = 0;
    let mut product = 0;
    let shift = Position { x: 1, y: 1 };
    let limit = Position { x: 20, y: 20 };

    let mut positions = [
        Position { x: 1, y: 1 },
        Position { x: 1, y: 2 },
        Position { x: 1, y: 3 },
    ];

    println!("Max product: {}", max_product)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// fn find_max_product(matrix: &Vec<Vec<u32>>, shift: &position) {
//     while are_valid_index(&positions, &limit) {
//         product = calculate_product(&matrix, &positions);
//         max_product = if max_product < product {
//             product
//         } else {
//             max_product
//         };
//         update_positions(&mut positions, &shift)
//     }
// }
