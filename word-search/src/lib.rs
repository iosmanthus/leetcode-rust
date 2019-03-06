pub struct Solution {}
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.bytes().collect();
        let mut board: Vec<Vec<_>> = board
            .into_iter()
            .map(|vec| vec.into_iter().map(|x| x as u8).collect())
            .collect();

        let sources: Vec<_> = board
            .iter()
            .enumerate()
            .map(|(i, vec)| {
                vec.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == word[0])
                    .map(|(j, _)| (i, j))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        fn adjacents(src: (usize, usize), board: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
            let mut adjacents = vec![];
            let (i, j) = src;
            if i > 0 {
                adjacents.push((i - 1, j));
            }
            if i + 1 < board.len() {
                adjacents.push((i + 1, j));
            }
            if j > 0 {
                adjacents.push((i, j - 1));
            }
            if j + 1 < board[0].len() {
                adjacents.push((i, j + 1));
            }
            adjacents
        }

        fn dfs(src: (usize, usize), board: &mut Vec<Vec<u8>>, word: &[u8]) -> bool {
            if word.is_empty() {
                return true;
            }
            let (i, j) = src;
            board[i][j] ^= 128;

            for (m, n) in adjacents((i, j), board) {
                if board[m][n] == word[0]
                    && board[m][n] & 128 == 0
                    && dfs((m, n), board, &word[1..])
                {
                    return true;
                }
            }

            board[i][j] ^= 128;
            false
        }

        for (i, j) in sources {
            if dfs((i, j), &mut board, &word[1..]) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        dbg!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ));
    }
}
