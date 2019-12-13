pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        assert!(Self::solve(board, 0));
    }

    // Return true if this sudoku board can be solved.
    fn solve(board: &mut Vec<Vec<char>>, mut filled: usize) -> bool {
        // Find a cell and try all options.
        while filled < 81 && board[filled / 9][filled % 9] != '.' {
            filled += 1;
        }

        // If all cells are filled, return true
        // indicating the sudoku board if solved.
        if filled == 81 {
            return true;
        }

        let row = filled / 9;
        let col = filled % 9;

        // Get all feasible options.
        let mut options = Self::options_at(row, col, board);

        let mut option = 1;
        while options != 0 {
            // Try for each option.
            if options & 1 == 1 {
                board[row][col] = char::from(option + b'0');
                if Self::solve(board, filled + 1) {
                    return true;
                }
                board[row][col] = '.';
            }

            option += 1;
            options >>= 1;
        }

        false
    }

    fn options_at(row: usize, col: usize, board: &Vec<Vec<char>>) -> u32 {
        let mut bits = 0b111111111;
        for i in 0..9 {
            // Remove numbers exist in the same row.
            bits &= !(((board[row][i] != '.') as u32)
                << (board[row][i] as u8).saturating_sub(b'0' + 1));
            // Remove numbers exist in the same column.
            bits &= !(((board[i][col] != '.') as u32)
                << (board[i][col] as u8).saturating_sub(b'0' + 1));

            // Remove numbers exist in the same 3x3 box.
            let x = row / 3 * 3 + i / 3;
            let y = col / 3 * 3 + i % 3;
            bits &=
                !(((board[x][y] != '.') as u32) << ((board[x][y] as u8).saturating_sub(b'0' + 1)));
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let cases = vec![(
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ],
        )];
        for (mut board, expected) in cases {
            Solution::solve_sudoku(&mut board);
            assert_eq!(board, expected);
        }
    }
}
