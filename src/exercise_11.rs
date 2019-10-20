use super::position::Position;

// Should be moved to position lib?
fn is_valid_index(position: &Position, matrix: &Vec<Vec<u32>>) -> bool {
    (matrix[0].len() > position.x) && (matrix.len() > position.y)
}

fn update_positions(positions: &mut [Position; 3], shift: &Position) {
    for position in positions {
        position.shift(shift)
    }
}

fn print_positions(positions: &[Position; 3]) {
    println!("{}, {}, {}", positions[0], positions[1], positions[2])
}

fn calculate_product(matrix: &Vec<Vec<u32>>, positions: &[Position; 3]) -> u32 {
    let mut acc = 1;
    for position in positions {
        acc *= matrix[position.y][position.x]
    }
    acc
}

fn init_positions(origin: &Position, shift: &Position) -> [Position; 3] {
    [
        Position::new(origin, shift, 0),
        Position::new(origin, shift, 1),
        Position::new(origin, shift, 2),
    ]
}

fn choose_bigger(v1: u32, v2: u32) -> u32 {
    let result = if v1 < v2 { v2 } else { v1 };
    result
}

fn find_max_product(matrix: &Vec<Vec<u32>>, origin: &Position, shift: &Position) -> u32 {
    let mut positions = init_positions(origin, shift);
    let mut max_product = 0;
    let mut product;

    while is_valid_index(&positions[2], &matrix) {
        product = calculate_product(&matrix, &positions);
        max_product = choose_bigger(max_product, product);
        update_positions(&mut positions, &shift)
    }
    max_product
}

fn find_max_product_with_origin_swap(
    matrix: &Vec<Vec<u32>>,
    swap_origin: &Position,
    shift: &Position,
) -> u32 {
    let mut origin = Position { x: 0, y: 0 };
    let mut max_product = 0;
    while is_valid_index(&origin, &matrix) {
        max_product = choose_bigger(max_product, find_max_product(&matrix, &origin, &shift));
        origin.shift(&swap_origin);
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn init_positions() {
        let origin = Position { x: 0, y: 0 };
        let shift = Position { x: 1, y: 1 };
        let positions = super::init_positions(&origin, &shift);
        assert_eq!(positions[0], Position { x: 0, y: 0 });
        assert_eq!(positions[1], Position { x: 1, y: 1 });
        assert_eq!(positions[2], Position { x: 2, y: 2 });
    }

    #[test]
    fn init_positions_with_shifted_origin() {
        let origin = Position { x: 1, y: 1 };
        let shift = Position { x: 1, y: 0 };
        let positions = super::init_positions(&origin, &shift);
        assert_eq!(positions[0], Position { x: 1, y: 1 });
        assert_eq!(positions[1], Position { x: 2, y: 1 });
        assert_eq!(positions[2], Position { x: 3, y: 1 });
    }

    #[test]
    fn horizontal() {
        let origin = Position { x: 0, y: 0 };
        let shift = Position { x: 1, y: 0 };
        let matrix = vec![vec![1, 2, 3, 4, 5, 5, 5, 1, 1, 1]];
        assert_eq!(super::find_max_product(&matrix, &origin, &shift), 125);
    }

    #[test]
    fn main_diagonal() {
        let origin = Position { x: 0, y: 0 };
        let matrix = vec![
            vec![5, 2, 3, 1, 1],
            vec![1, 2, 3, 1, 1],
            vec![1, 2, 3, 1, 1],
            vec![1, 2, 3, 3, 1],
            vec![1, 2, 3, 1, 1],
        ];
        let shift = Position { x: 1, y: 1 };
        assert_eq!(super::find_max_product(&matrix, &origin, &shift), 30);
    }

    #[test]
    fn all_horizontal() {
        let swap_origin = Position { x: 0, y: 1 };
        let shift = Position { x: 1, y: 0 };

        let matrix = vec![
            vec![5, 2, 3, 1, 1],
            vec![1, 2, 3, 1, 1],
            vec![1, 2, 3, 1, 1],
            vec![10, 2, 3, 3, 1],
            vec![1, 2, 3, 1, 1],
        ];

        assert_eq!(
            super::find_max_product_with_origin_swap(&matrix, &swap_origin, &shift),
            60
        )
    }
}
