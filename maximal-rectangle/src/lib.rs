pub struct Solution {}
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        use std::cmp;

        if matrix.is_empty() {
            return 0;
        }

        let mut heights = vec![0; matrix[0].len()];
        let mut max = 0;

        for row in matrix {
            for (i, ch) in row.into_iter().enumerate() {
                if ch == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }
            let mut stack = vec![];

            let mut i = 1;
            stack.push(0);
            while i < heights.len() {
                if stack.is_empty() || heights[*stack.last().unwrap()] <= heights[i] {
                    stack.push(i);
                    i += 1;
                } else {
                    let top = *stack.last().unwrap();
                    let _ = stack.pop().unwrap();
                    max = cmp::max(
                        max,
                        heights[top] * if stack.is_empty() {
                            i
                        } else {
                            i - stack.last().unwrap() - 1
                        } as i32,
                    );
                }
            }
            while !stack.is_empty() {
                let top = *stack.last().unwrap();
                let _ = stack.pop().unwrap();
                max = cmp::max(
                    max,
                    heights[top] * if stack.is_empty() {
                        i
                    } else {
                        i - stack.last().unwrap() - 1
                    } as i32,
                );
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        );
    }
}
