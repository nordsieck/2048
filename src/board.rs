use std::fmt;

enum Direction { Up, Down, Left, Right }

#[derive(Debug, Eq)]
struct State { board: [[u32; 4]; 4], score: u32 }

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool { self.board == other.board && self.score == other.score }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:8}{:8}{:8}{:8}\n{:8}{:8}{:8}{:8}\n{:8}{:8}{:8}{:8}\n{:8}{:8}{:8}{:8}",
               self.board[0][0], self.board[0][1], self.board[0][2], self.board[0][3],
               self.board[1][0], self.board[1][1], self.board[1][2], self.board[1][3],
               self.board[2][0], self.board[2][1], self.board[2][2], self.board[2][3],
               self.board[3][0], self.board[3][1], self.board[3][2], self.board[3][3])
    }
}

fn clone_state(b: &State) -> State {
    let mut dst = new_board();
    dst.board.copy_from_slice(&b.board);
    dst.score = b.score;
    dst
}

fn new_board() -> State { return State{ board: [[0; 4]; 4], score: 0 }}

fn add_piece(b: &State, get_pos: &Fn(&State) -> Option<(usize, usize)>, get_val: &Fn() -> u32) -> Option<State> {
    match get_pos(b) {
        Some((r, c)) => {
            let mut next = clone_state(b);
            next.board[r][c] = get_val();
            Some(next)
        },
        None => None,
    }
}

fn move_board(b: &State, d: Direction) -> Option<State> {
    let (mut next, mut modified) = (clone_state(b), false);
    
    match d {
        Direction::Up => {
            for c in 0..4 {
                let (mut free, mut last) = (0, 0);
                for r in 0..4 {
                    if next.board[r][c] != 0 {
                        if r != last && next.board[r][c] == next.board[last][c] {
                            next.board[last][c] *= 2;
                            next.score += next.board[last][c];
                            next.board[r][c] = 0;
                            last += 1;
                            modified = true;
                        } else if r != free {
                            let tmp = next.board[free][c];
                            next.board[free][c] = next.board[r][c];
                            next.board[r][c] = tmp;
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
                    if next.board[r][c] != 0 {
                        if r != last && next.board[r][c] == next.board[last][c] {
                            next.board[last][c] *= 2;
                            next.score += next.board[last][c];
                            next.board[r][c] = 0;
                            last = if last > 0 { last - 1 } else { last }; // ?
                            modified = true;
                        } else if r != free {
                            let tmp = next.board[free][c];
                            next.board[free][c] = next.board[r][c];
                            next.board[r][c] = tmp;
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
                    if next.board[r][c] != 0 {
                        if c != last && next.board[r][c] == next.board[r][last] {
                            next.board[r][last] *= 2;
                            next.score += next.board[r][last];
                            next.board[r][c] = 0;
                            last += 1;
                            modified = true;
                        } else if c != free {
                            let tmp = next.board[r][free];
                            next.board[r][free] = next.board[r][c];
                            next.board[r][c] = tmp;
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
                    if next.board[r][c] != 0 {
                        if c != last && next.board[r][c] == next.board[r][last] {
                            next.board[r][last] *= 2;
                            next.score += next.board[r][last];
                            next.board[r][c] = 0;
                            last  = if last > 0 { last - 1 } else { last }; // ?
                            modified = true;
                        } else if c != free {
                            let tmp = next.board[r][free];
                            next.board[r][free] = next.board[r][c];
                            next.board[r][c] = tmp;
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
    if modified { Some(next) } else { None }
}

fn new_val_2() -> u32 { 2 }

fn low_pos(b: &State) -> Option<(usize, usize)> {
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
        assert_eq!(b, State{ board:[[0; 4]; 4], score: 0 })
    }

    #[test]
    fn test_low_pos() {
        fn t(b: State, r: Option<(usize, usize)>) { assert_eq!(low_pos(&b), r); }

        t(new_board(), Some((0, 0)));
        t(State{board:[[1; 4], [1; 4], [1; 4], [1, 1, 1, 0]], score: 0}, Some((3, 3)));
        t(State{board:[[1; 4]; 4], score: 0}, None);
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", new_board()), "       0       0       0       0
       0       0       0       0
       0       0       0       0
       0       0       0       0")
    }
    
    #[test]
    fn test_add_piece() {
        let b = add_piece(&new_board(), &low_pos, &new_val_2);
        assert_eq!(b, Some(State{board:[[2, 0, 0, 0], [0; 4], [0; 4], [0; 4]], score: 0}));
    }

    #[test]
    fn test_move_board() {
        fn t(direction: Direction, board: State, result: Option<State>) {
            let b = move_board(&board, direction);
            assert_eq!(b, result);
        }

        t(Direction::Left, new_board(), None);
        t(Direction::Left, State{board: [[1, 0, 0, 0]; 4], score: 0}, None);
        t(Direction::Left, State{board: [[1, 2, 3, 4]; 4], score: 0}, None);
        t(Direction::Left, State{board: [[0, 0, 0, 1]; 4], score: 0}, Some(State{board: [[1, 0, 0, 0]; 4], score: 0}));
        t(Direction::Left, State{board: [[1, 0, 0, 1]; 4], score: 0}, Some(State{board: [[2, 0, 0, 0]; 4], score: 8}));
        t(Direction::Left, State{board: [[1, 1, 2, 2]; 4], score: 0}, Some(State{board: [[2, 4, 0, 0]; 4], score: 24}));

        t(Direction::Right, new_board(), None);
        t(Direction::Right, State{board: [[0, 0, 0, 1]; 4], score: 0}, None);
        t(Direction::Right, State{board: [[1, 2, 3, 4]; 4], score: 0}, None);
        t(Direction::Right, State{board: [[1, 0, 0, 0]; 4], score: 0}, Some(State{board: [[0, 0, 0, 1]; 4], score: 0}));
        t(Direction::Right, State{board: [[1, 0, 0, 1]; 4], score: 0}, Some(State{board: [[0, 0, 0, 2]; 4], score: 8}));
        t(Direction::Right, State{board: [[1, 1, 2, 2]; 4], score: 0}, Some(State{board: [[0, 0, 2, 4]; 4], score: 24}));

        t(Direction::Up, new_board(), None);
        t(Direction::Up, State{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}, None);
        t(Direction::Up, State{board: [[1; 4], [2; 4], [3; 4], [4; 4]], score: 0}, None);
        t(Direction::Up, State{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(State{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}));
        t(Direction::Up, State{board: [[1; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(State{board: [[2; 4], [0; 4], [0; 4], [0; 4]], score: 8}));
        t(Direction::Up, State{board: [[1; 4], [1; 4], [2; 4], [2; 4]], score: 0}, Some(State{board: [[2; 4], [4; 4], [0; 4], [0; 4]], score: 24}));

        t(Direction::Down, new_board(), None);
        t(Direction::Down, State{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}, None);
        t(Direction::Down, State{board: [[1; 4], [2; 4], [3; 4], [4; 4]], score: 0}, None);
        t(Direction::Down, State{board: [[1; 4], [0; 4], [0; 4], [0; 4]], score: 0}, Some(State{board: [[0; 4], [0; 4], [0; 4], [1; 4]], score: 0}));
        t(Direction::Down, State{board: [[1; 4], [0; 4], [0; 4], [1; 4]], score: 0}, Some(State{board: [[0; 4], [0; 4], [0; 4], [2; 4]], score: 8}));
        t(Direction::Down, State{board: [[1; 4], [1; 4], [2; 4], [2; 4]], score: 0}, Some(State{board: [[0; 4], [0; 4], [2; 4], [4; 4]], score: 24}));
    }
}
