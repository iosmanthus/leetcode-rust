pub struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (row, column) = (matrix.len() / 2, (matrix.len() + 1) / 2);
        let len = matrix.len();
        for i in 0..row {
            for j in 0..column {
                let mut prev = matrix[i][j];
                let (mut i, mut j) = (i, j);
                for _ in 0..4 {
                    let (m, n) = (j, len - 1 - i);
                    std::mem::swap(&mut prev, &mut matrix[m][n]);
                    i = m;
                    j = n;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ],
            matrix
        );

        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix);
    }
}
