use std::fmt;

pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn shift(&mut self, shift: &Position) {
        self.x += shift.x;
        self.y += shift.y;
    }
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
        position.shift(shift)
    }
}

// fn print_positions(positions: &[Position; 3]) {
//     println!("{}, {}, {}", positions[0], positions[1], positions[2])
// }

fn calculate_product(matrix: &Vec<Vec<u32>>, positions: &[Position; 3]) -> u32 {
    let mut acc = 1;
    for position in positions {
        acc *= matrix[position.y][position.x]
    }
    acc
}

// fn init_positions(shift: &Position) -> [Position; 3] {
//     let origin = Position { x: 0, y: 0 };
//     [
//         origin,
//         update_positions(&mut origin, shift),
//         update_positions(&mut update_positions(&mut origin, shift), shift),
//     ]
// }

fn find_max_product(matrix: &Vec<Vec<u32>>, shift: &Position) -> u32 {
    // build positions based on shift
    let mut positions = [
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 2, y: 0 },
    ];

    // init_positions(shift);

    let limit = Position {
        y: matrix.len(),
        x: matrix[0].len(),
    };

    let mut max_product = 0;
    let mut product;

    while are_valid_index(&positions, &limit) {
        product = calculate_product(&matrix, &positions);
        max_product = if max_product < product {
            product
        } else {
            max_product
        };
        update_positions(&mut positions, &shift)
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::Position;
    #[test]
    fn horizontal() {
        let shift = Position { x: 1, y: 0 };
        let matrix = vec![vec![1, 2, 3, 4, 5, 5, 5, 1, 1, 1]];
        assert_eq!(super::find_max_product(&matrix, &shift), 125);
    }

    // #[test]
    // fn init_positions() {
    //     let shift = Position { x: 1, y: 0 };
    //     println!("Testing init positions");
    //     super::init_positions(&shift);
    //     assert_eq!(56, 67);
    // }

}
