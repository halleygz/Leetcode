impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &Vec<Vec<char>>, mask: &mut Vec<Vec<bool>>, row: usize, col: usize) {
            if !mask[row][col] && board[row][col]=='O' {
                mask[row][col] = true;
            } else {
                return;
            }

            if row>0 {
                dfs(board, mask, row-1, col);
            }
            if row<board.len()-1 {
                dfs(board, mask, row+1, col);
            }
            if col>0 {
                dfs(board, mask, row, col-1);
            }
            if col<board[0].len()-1 {
                dfs(board, mask, row, col+1);
            }
        }
        
        let nrow = board.len();
        let ncol = board[0].len();
        let mut mask = vec![vec![false; ncol]; nrow];

        for row in 0..nrow {
            dfs(&board, &mut mask, row, 0);
            dfs(&board, &mut mask, row, ncol-1);
        }

        for col in 0..ncol {
            dfs(&board, &mut mask, 0, col);
            dfs(&board, &mut mask, nrow-1, col);
        }
        
        for row in 0..nrow {
            for col in 0..ncol {
                if !mask[row][col] {
                    board[row][col] = 'X';
                }
            }
        }
    }
}