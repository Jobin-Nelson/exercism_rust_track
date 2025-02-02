const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut movement = VECTORS.iter().cycle();
    let (mut x, mut y, mut n) = (-1, 0, 1);
    for (move_x, move_y) in std::iter::once(size)
        .chain((1..size).rev().flat_map(|n| std::iter::repeat(n).take(2)))
        .flat_map(|step| std::iter::repeat(movement.next().unwrap()).take(step))
    {
        x += move_x;
        y += move_y;
        matrix[y as usize][x as usize] = n;
        n += 1;
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::spiral_matrix;

    #[test]
    fn empty_spiral() {
        let input = 0;
        let output = spiral_matrix(input);
        let expected: [[u32; 0]; 0] = [];
        assert_eq!(output, expected);
    }
    #[test]
    fn trivial_spiral() {
        let input = 1;
        let output = spiral_matrix(input);
        let expected: [[u32; 1]; 1] = [[1]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_2() {
        let input = 2;
        let output = spiral_matrix(input);
        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_3() {
        let input = 3;
        let output = spiral_matrix(input);
        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_4() {
        let input = 4;
        let output = spiral_matrix(input);
        let expected: [[u32; 4]; 4] = [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7],
        ];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_5() {
        let input = 5;
        let output = spiral_matrix(input);
        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        assert_eq!(output, expected);
    }
}
