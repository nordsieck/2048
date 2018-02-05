use std::collections::HashMap;

enum Direction { Up, Down, Left, Right }

#[derive(Debug)]
struct Board { board: [[u32; 4]; 4], score: u32 }

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool { self.board == other.board && self.score == other.score }
}

fn clone_board(b: &Board) -> Board {
    let mut dst = new_board();
    dst.board.copy_from_slice(&b.board);
    dst.score = b.score;
    dst
}

fn new_board() -> Board { return Board{ board: [[0; 4]; 4], score: 0 }}

fn add_piece(b: &Board, get_pos: &Fn(&Board) -> Option<(usize, usize)>, get_val: &Fn() -> u32) -> Option<Board> {
    match get_pos(b) {
        Some((r, c)) => {
            let mut board = clone_board(b);
            board.board[r][c] = get_val();
            Some(board)
        },
        None => None,
    }
}

fn move_board(b: &Board, d: Direction) -> Option<Board> {
    let (mut board, mut modified) = (clone_board(b), false);
    
    match d {
        Direction::Up => {
            for c in 0..4 {
                let (mut free, mut last) = (0, 0);
                for r in 0..4 {
                    if board.board[r][c] != 0 {
                        if r != last && board.board[r][c] == board.board[last][c] {
                            board.board[last][c] *= 2;
                            board.score += board.board[last][c];
                            board.board[r][c] = 0;
                            last += 1;
                            modified = true;
                        } else if r != free {
                            let tmp = board.board[free][c];
                            board.board[free][c] = board.board[r][c];
                            board.board[r][c] = tmp;
                            modified = true;
                            free += 1;
                        } else {
                            free += 1;
                        }
                    }
                }
            }
        },
        Direction::Down => {
            for c in 0..4 {
                let (mut free, mut last) = (3, 3);
                for r in (0..4).rev() {
                    if board.board[r][c] != 0 {
                        if r != last && board.board[r][c] == board.board[last][c] {
                            board.board[last][c] *= 2;
                            board.score += board.board[last][c];
                            board.board[r][c] = 0;
                            last = if last > 0 { last - 1 } else { last }; // ?
                            modified = true;
                        } else if r != free {
                            let tmp = board.board[free][c];
                            board.board[free][c] = board.board[r][c];
                            board.board[r][c] = tmp;
                            modified = true;
                            free = if free > 0 { free - 1 } else { free }; // ?
                        } else {
                            free = if free > 0 { free - 1 } else { free }; // ?
                        }
                    }
                }
            }
        },
        Direction::Left => {
            for r in 0..4 {
                let (mut free, mut last) = (0, 0);
                for c in 0..4 {
                    if board.board[r][c] != 0 {
                        if c != last && board.board[r][c] == board.board[r][last] {
                            board.board[r][last] *= 2;
                            board.score += board.board[r][last];
                            board.board[r][c] = 0;
                            last += 1;
                            modified = true;
                        } else if c != free {
                            let tmp = board.board[r][free];
                            board.board[r][free] = board.board[r][c];
                            board.board[r][c] = tmp;
                            modified = true;
                            free += 1;
                        } else {
                            free += 1;
                        }
                    }
                }
            }
        },
        Direction::Right => {
            for r in 0..4 {
                let (mut free, mut last) = (3, 3);
                for c in (0..4).rev() {
                    if board.board[r][c] != 0 {
                        if c != last && board.board[r][c] == board.board[r][last] {
                            board.board[r][last] *= 2;
                            board.score += board.board[r][last];
                            board.board[r][c] = 0;
                            last  = if last > 0 { last - 1 } else { last }; // ?
                            modified = true;
                        } else if c != free {
                            let tmp = board.board[r][free];
                            board.board[r][free] = board.board[r][c];
                            board.board[r][c] = tmp;
                            modified = true;
                            free = if free > 0 { free - 1 } else { free }; // ?
                        } else {
                            free = if free > 0 { free - 1 } else { free }; // ?
                        }
                    }
                }
            }
        },
    }
    if modified { Some(board) } else { None }
}

fn new_val_2() -> u32 { 2 }

fn low_pos(b: &Board) -> Option<(usize, usize)> {
    for r in 0 .. 4 {
        for c in 0 .. 4 {
            if b.board[r][c] == 0 { return Some((r, c)); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_board() {
        let b = new_board();
        assert_eq!(b, Board{ board:[[0; 4]; 4], score: 0 })
    }

    #[test]
    fn test_low_pos() {
        fn t(b: Board, r: Option<(usize, usize)>) { assert_eq!(low_pos(&b), r); }

        t(new_board(), Some((0, 0)));
        t(Board{board:[[1; 4], [1; 4], [1; 4], [1, 1, 1, 0]], score: 0}, Some((3, 3)));
        t(Board{board:[[1; 4]; 4], score: 0}, None);
    }

    #[test]
    fn test_add_piece() {
        let b = add_piece(&new_board(), &low_pos, &new_val_2);
        assert_eq!(b, Some(Board{board:[[2, 0, 0, 0], [0; 4], [0; 4], [0; 4]], score: 0}));
    }

    #[test]
    fn test_move_board() {
        fn t(direction: Direction, board: Board, result: Option<Board>) {
            let b = move_board(&board, direction);
            assert_eq!(b, result);
        }

        t(Direction::Left, new_board(), None);
        t(Direction::Left, Board{board: [[1, 0, 0, 0]; 4], score: 0}, None);
        t(Direction::Left, Board{board: [[1, 2, 3, 4]; 4], score: 0}, None);
        t(Direction::Left, Board{board: [[0, 0, 0, 1]; 4], score: 0}, Some(Board{board: [[1, 0, 0, 0]; 4], score: 0}));
        t(Direction::Left, Board{board: [[1, 0, 0, 1]; 4], score: 0}, Some(Board{board: [[2, 0, 0, 0]; 4], score: 8}));
        t(Direction::Left, Board{board: [[1, 1, 2, 2]; 4], score: 0}, Some(Board{board: [[2, 4, 0, 0]; 4], score: 24}));

        t(Direction::Right, new_board(), None);
        t(Direction::Right, Board{board: [[0, 0, 0, 1]; 4], score: 0}, None);
        t(Direction::Right, Board{board: [[1, 2, 3, 4]; 4], score: 0}, None);
        t(Direction::Right, Board{board: [[1, 0, 0, 0]; 4], score: 0}, Some(Board{board: [[0, 0, 0, 1]; 4], score: 0}));
        t(Direction::Right, Board{board: [[1, 0, 0, 1]; 4], score: 0}, Some(Board{board: [[0, 0, 0, 2]; 4], score: 8}));
        t(Direction::Right, Board{board: [[1, 1, 2, 2]; 4], score: 0}, Some(Board{board: [[0, 0, 2, 4]; 4], score: 24}));

        t(Direction::Up, new_board(), None);
        t(Direction::Up, Board{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}, None);
        t(Direction::Up, Board{board: [[1; 4], [2; 4], [3; 4], [4; 4]], score: 0}, None);
        t(Direction::Up, Board{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(Board{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}));
        t(Direction::Up, Board{board: [[1; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(Board{board: [[2; 4], [0; 4], [0; 4], [0; 4]], score: 8}));
        t(Direction::Up, Board{board: [[1; 4], [1; 4], [2; 4], [2; 4]], score: 0}, Some(Board{board: [[2; 4], [4; 4], [0; 4], [0; 4]], score: 24}));

        t(Direction::Down, new_board(), None);
        t(Direction::Down, Board{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}, None);
        t(Direction::Down, Board{board: [[1; 4], [2; 4], [3; 4], [4; 4]], score: 0}, None);
        t(Direction::Down, Board{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}, Some(Board{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}));
        t(Direction::Down, Board{board: [[1; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(Board{board: [[0; 4], [0; 4], [0; 4], [2; 4]], score: 8}));
        t(Direction::Down, Board{board: [[1; 4], [1; 4], [2; 4], [2; 4]], score: 0}, Some(Board{board: [[0; 4], [0; 4], [2; 4], [4; 4]], score: 24}));
    }
}
