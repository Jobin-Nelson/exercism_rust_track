#[rustfmt::skip]
const DIRS: &[(i32, i32); 8] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // let height = minefield.len() as i32;
    // (0..height).map(|y| {
    //     let width = minefield[y as usize].len() as i32;
    //     (0..width).map(|x| {
    //         if minefield[y as usize].as_bytes()[x as usize] == b'*' {
    //             '*'
    //         } else {
    //             match DIRS.iter()
    //                 .map(|&d| (x + d.0, y + d.1))
    //                 .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
    //                 .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
    //                 .count() {
    //                     0 => ' ',
    //                     n => (n as u8 + '0' as u8) as char
    //                 }
    //         }
    //     }).collect()
    // }).collect()

    let mine_vec: Vec<Vec<char>> = minefield
        .iter()
        .map(|r| r.chars().collect())
        .collect();

    let mut result = Vec::new();

    let row_len = mine_vec.len();
    let col_len = if let Some(r) = mine_vec.get(0) {
        r.len()
    } else {
        0
    };

    for r in 0..row_len {
        let mut new_row = String::new();
        for c in 0..col_len {
            let value = mine_vec[r][c];
            if value == ' ' {
                let mut count = 0;
                for d in DIRS {
                    let (dx, dy) = (r as i32 + d.0, c as i32 + d.1);
                    if let Some(nei) = mine_vec.get(dx as usize).and_then(|v| v.get(dy as usize)) {
                        if nei == &'*' {
                            count += 1;
                        }
                    }
                }
                if count == 0 {
                    new_row.push(' ');
                } else {
                    new_row.push((count as u8 + '0' as u8) as char);
                }
            } else {
                new_row.push(value)
            }
        }
        result.push(new_row)
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }

    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }

    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }

    #[test]
    fn no_rows() {
        #[rustfmt::skip]
        run_test(&[
        ]);
    }

    #[test]
    fn no_columns() {
        #[rustfmt::skip]
        run_test(&[
             "",
        ]);
    }

    #[test]
    fn no_mines() {
        #[rustfmt::skip]
        run_test(&[
             "   ",
             "   ",
             "   ",
        ]);
    }

    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
        run_test(&[
             "***",
             "***",
             "***",
        ]);
    }

    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
        run_test(&[
             "***",
             "*8*",
             "***",
        ]);
    }

    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
        run_test(&[
             "***",
             "***",
             "***",
        ]);
    }
}
