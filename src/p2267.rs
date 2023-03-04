/// # 2267. Check if There Is a Valid Parentheses String Path
///
/// [Link][link]
///
/// Meet in the middle breadth-first search, though depth-first search could've also worked just as
/// well. Most of the edge cases I caught ahead of time (the ones that return false early on), but
/// the case where it's a simple domino ([['(',')']] or [['('],[')']]) my solution didn't catch
/// because states are getting added to `middle_states` when they're encountered, not when they're
/// popped. Alternatively, I could've moved `process_state` (the first one) out of the while loop,
/// and passed `end` into there, but the signature would need to change so that `queue` wasn't
/// borrowed mutably twice. I also didn't consider the duplicate states that would be encountered,
/// so `seen_states` was added after failing due to time.
///
/// [link]: https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/
pub struct Solution;

use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct State {
    x: u8,
    y: u8,
    open: i8,
}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n): (u8, u8) = (grid[0_usize].len() as u8, grid.len() as u8);
        let end: State = State {
            x: m - 1_u8,
            y: n - 1_u8,
            open: 0_i8,
        };
        let path_len: u8 = m + n - 1_u8;
        let opens = |x: u8, y: u8| -> bool { grid[y as usize][x as usize] == '(' };
        let opens_delta = |x: u8, y: u8| -> i8 {
            if opens(x, y) {
                1_i8
            } else {
                -1_i8
            }
        };

        if path_len % 2_u8 != 0_u8 || !opens(0_u8, 0_u8) || opens(end.x, end.y) {
            return false;
        }

        if path_len == 2_u8 {
            return true;
        }

        let half_path_len: u8 = path_len >> 1_u32;
        let mut seen_states: HashSet<State> = HashSet::new();
        let mut middle_states: HashSet<State> = HashSet::new();
        let mut queue: VecDeque<State> = VecDeque::new();

        // Build up `middle_states`
        queue.push_back(end);

        while let Some(state) = queue.pop_front() {
            let open: i8 = state.open - opens_delta(state.x, state.y);

            let mut process_state = |state: State| {
                if seen_states.insert(state) && (state.open != 0_i8 || !opens(state.x, state.y)) {
                    if state.x + state.y == half_path_len {
                        middle_states.insert(state);
                    } else {
                        queue.push_back(state);
                    }
                }
            };

            if open >= 0_i8 {
                if let Some(x) = state.x.checked_sub(1_u8) {
                    let y: u8 = state.y;

                    process_state(State { x, y, open });
                }

                if let Some(y) = state.y.checked_sub(1_u8) {
                    let x: u8 = state.x;

                    process_state(State { x, y, open });
                }
            }
        }

        if middle_states.is_empty() {
            return false;
        }

        seen_states.clear();

        // Find a state in `middle_states`
        queue.push_back(State {
            x: 0_u8,
            y: 0_u8,
            open: 1_i8,
        });

        while let Some(state) = queue.pop_front() {
            let mut process_state = |state: State| -> bool {
                if seen_states.insert(state) {
                    if state.x + state.y == half_path_len {
                        middle_states.contains(&state)
                    } else {
                        if state.open >= 0_i8 {
                            queue.push_back(state);
                        }

                        false
                    }
                } else {
                    false
                }
            };

            let x: u8 = state.x + 1_u8;

            if x < m {
                let y: u8 = state.y;
                let open: i8 = state.open + opens_delta(x, y);

                if process_state(State { x, y, open }) {
                    return true;
                }
            }

            let y: u8 = state.y + 1_u8;

            if y < n {
                let x: u8 = state.x;
                let open: i8 = state.open + opens_delta(x, y);

                if process_state(State { x, y, open }) {
                    return true;
                }
            }
        }

        false
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid: Vec<Vec<char>> = vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')'],
        ];

        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<char>> = vec![vec![')', ')'], vec!['(', '(']];

        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn example_3() {
        let grid: Vec<Vec<char>> = vec![vec!['(', ')'], vec!['(', ')']];

        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn example_4() {
        let grid: Vec<Vec<char>> = vec![vec![')', ')'], vec!['(', ')']];

        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn example_5() {
        let grid: Vec<Vec<char>> = vec![vec!['(', ')'], vec!['(', '(']];

        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn example_6() {
        let grid: Vec<Vec<char>> = vec![
            vec!['(', ')', ')'],
            vec![')', ')', '('],
            vec!['(', '(', '('],
            vec![')', '(', ')'],
        ];

        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn example_7() {
        let grid: Vec<Vec<char>> = vec![
            vec![
                '(', ')', ')', ')', ')', ')', ')', ')', '(', ')', '(', ')', ')', ')', '(', ')',
                ')', '(', ')', '(', ')', '(', ')', '(', '(', '(', '(', ')', '(', ')', ')', '(',
                ')', '(', '(', '(', '(', '(', '(', ')', ')', ')', '(', ')', '(', '(', '(', '(',
                '(', ')',
            ],
            vec![
                '(', ')', '(', ')', ')', ')', ')', ')', '(', '(', ')', '(', '(', ')', '(', ')',
                '(', '(', '(', ')', '(', ')', ')', ')', '(', ')', '(', '(', '(', ')', ')', '(',
                ')', '(', '(', '(', '(', '(', '(', '(', '(', ')', ')', '(', '(', ')', ')', '(',
                '(', ')',
            ],
            vec![
                '(', '(', '(', ')', ')', '(', '(', '(', ')', ')', '(', ')', '(', ')', ')', ')',
                '(', '(', '(', '(', '(', ')', ')', '(', ')', '(', '(', ')', '(', ')', '(', ')',
                '(', ')', '(', '(', ')', '(', ')', '(', '(', ')', ')', '(', ')', ')', '(', ')',
                ')', '(',
            ],
            vec![
                ')', ')', '(', '(', ')', ')', '(', ')', ')', '(', '(', '(', ')', '(', '(', '(',
                '(', ')', ')', ')', ')', ')', '(', ')', ')', '(', ')', ')', ')', '(', ')', '(',
                '(', ')', '(', ')', ')', ')', '(', '(', ')', ')', ')', ')', '(', ')', ')', ')',
                '(', '(',
            ],
            vec![
                '(', ')', '(', ')', ')', '(', '(', ')', '(', ')', '(', '(', '(', ')', ')', ')',
                '(', '(', '(', ')', '(', '(', ')', '(', ')', '(', ')', ')', '(', '(', ')', ')',
                '(', ')', ')', ')', ')', ')', ')', '(', '(', ')', ')', '(', ')', '(', '(', ')',
                '(', '(',
            ],
            vec![
                ')', ')', ')', ')', '(', '(', ')', ')', '(', ')', ')', '(', ')', ')', '(', ')',
                '(', '(', '(', '(', ')', ')', '(', '(', '(', ')', '(', ')', '(', '(', '(', '(',
                ')', '(', '(', ')', ')', ')', '(', '(', ')', '(', '(', ')', ')', ')', '(', ')',
                '(', '(',
            ],
            vec![
                '(', '(', ')', '(', '(', ')', ')', '(', ')', '(', '(', ')', '(', ')', ')', ')',
                ')', '(', '(', '(', '(', '(', ')', '(', '(', ')', ')', ')', ')', ')', '(', ')',
                '(', ')', ')', '(', '(', ')', ')', ')', '(', ')', ')', '(', ')', ')', ')', ')',
                ')', '(',
            ],
            vec![
                ')', ')', ')', '(', '(', '(', ')', ')', '(', ')', ')', ')', '(', '(', '(', '(',
                ')', ')', ')', '(', ')', ')', ')', ')', ')', ')', ')', '(', ')', ')', '(', ')',
                '(', ')', ')', '(', '(', '(', '(', '(', ')', '(', ')', ')', ')', ')', ')', ')',
                ')', ')',
            ],
            vec![
                '(', ')', '(', '(', ')', ')', ')', ')', ')', ')', ')', '(', ')', '(', '(', '(',
                '(', ')', ')', '(', ')', ')', ')', '(', ')', '(', ')', ')', '(', '(', ')', '(',
                ')', ')', ')', '(', ')', '(', '(', '(', ')', ')', ')', '(', ')', '(', '(', ')',
                ')', ')',
            ],
            vec![
                ')', '(', '(', ')', ')', '(', '(', ')', ')', ')', ')', '(', '(', '(', ')', '(',
                '(', ')', ')', ')', '(', '(', '(', '(', ')', ')', ')', '(', '(', ')', ')', ')',
                ')', ')', '(', '(', '(', '(', ')', ')', ')', '(', ')', '(', ')', '(', '(', '(',
                '(', ')',
            ],
            vec![
                '(', '(', '(', ')', '(', '(', ')', '(', '(', ')', '(', '(', '(', '(', '(', '(',
                '(', ')', ')', '(', '(', '(', '(', ')', '(', '(', '(', ')', ')', '(', '(', '(',
                '(', '(', '(', '(', '(', ')', ')', ')', ')', '(', ')', '(', ')', '(', ')', '(',
                '(', '(',
            ],
            vec![
                '(', ')', ')', '(', ')', ')', '(', ')', ')', '(', ')', ')', ')', '(', ')', '(',
                '(', '(', '(', '(', '(', '(', ')', ')', ')', '(', ')', ')', ')', ')', ')', ')',
                '(', '(', '(', '(', ')', '(', ')', '(', ')', '(', ')', '(', '(', ')', '(', '(',
                '(', ')',
            ],
            vec![
                ')', '(', ')', ')', ')', ')', '(', ')', '(', ')', '(', ')', '(', '(', ')', '(',
                '(', ')', ')', '(', ')', ')', ')', '(', ')', '(', '(', '(', ')', ')', ')', ')',
                '(', ')', '(', ')', ')', '(', ')', '(', '(', ')', '(', ')', ')', ')', '(', '(',
                ')', ')',
            ],
            vec![
                '(', '(', ')', ')', ')', ')', ')', '(', ')', ')', '(', ')', '(', ')', ')', '(',
                ')', '(', '(', ')', ')', '(', ')', '(', ')', '(', ')', '(', '(', '(', '(', '(',
                ')', '(', ')', ')', '(', '(', '(', ')', ')', ')', '(', '(', ')', '(', '(', '(',
                '(', ')',
            ],
            vec![
                ')', '(', ')', ')', '(', '(', ')', '(', ')', ')', ')', ')', ')', '(', ')', '(',
                '(', ')', '(', ')', ')', ')', '(', '(', '(', '(', ')', ')', '(', '(', ')', ')',
                '(', ')', ')', ')', '(', ')', '(', '(', '(', '(', ')', ')', ')', ')', '(', ')',
                '(', '(',
            ],
            vec![
                ')', ')', '(', ')', '(', '(', '(', '(', ')', '(', ')', '(', ')', '(', '(', '(',
                ')', '(', ')', ')', ')', ')', '(', '(', ')', '(', ')', ')', ')', ')', ')', ')',
                '(', '(', ')', '(', ')', ')', '(', ')', '(', ')', ')', ')', ')', ')', '(', '(',
                ')', '(',
            ],
            vec![
                '(', ')', ')', '(', '(', '(', '(', '(', '(', '(', '(', '(', '(', '(', ')', '(',
                ')', '(', '(', '(', ')', ')', ')', '(', ')', '(', ')', '(', '(', ')', '(', '(',
                '(', '(', '(', '(', '(', ')', ')', '(', ')', ')', '(', ')', '(', '(', '(', ')',
                ')', '(',
            ],
            vec![
                '(', '(', ')', ')', '(', ')', '(', '(', '(', '(', '(', '(', '(', ')', ')', '(',
                ')', '(', '(', '(', ')', '(', '(', '(', ')', ')', '(', '(', ')', ')', '(', ')',
                '(', '(', ')', ')', ')', ')', '(', ')', ')', ')', ')', ')', ')', ')', ')', ')',
                '(', '(',
            ],
            vec![
                ')', '(', ')', ')', '(', ')', ')', '(', ')', '(', ')', ')', '(', '(', '(', ')',
                ')', '(', ')', '(', ')', ')', '(', ')', ')', '(', ')', '(', '(', '(', ')', '(',
                '(', ')', '(', '(', '(', '(', '(', ')', '(', '(', ')', '(', '(', ')', '(', '(',
                ')', ')',
            ],
            vec![
                ')', '(', '(', '(', ')', '(', '(', ')', ')', '(', '(', '(', ')', '(', '(', ')',
                '(', ')', '(', '(', '(', '(', '(', '(', '(', ')', '(', '(', ')', ')', ')', ')',
                '(', '(', '(', '(', '(', ')', ')', ')', '(', '(', '(', ')', ')', ')', ')', '(',
                ')', '(',
            ],
            vec![
                ')', ')', ')', '(', ')', ')', ')', '(', ')', '(', '(', ')', ')', ')', '(', ')',
                '(', ')', ')', '(', '(', ')', ')', '(', '(', '(', ')', ')', '(', ')', ')', '(',
                ')', '(', '(', '(', ')', '(', ')', '(', ')', ')', '(', '(', ')', '(', '(', ')',
                '(', ')',
            ],
            vec![
                '(', '(', '(', ')', '(', ')', ')', ')', '(', ')', ')', ')', '(', '(', '(', ')',
                '(', '(', '(', '(', '(', ')', '(', ')', ')', ')', ')', '(', ')', '(', ')', '(',
                '(', ')', ')', ')', ')', '(', ')', '(', ')', ')', '(', '(', ')', '(', '(', ')',
                ')', '(',
            ],
            vec![
                '(', '(', ')', ')', ')', '(', ')', ')', '(', '(', ')', '(', '(', ')', ')', '(',
                ')', '(', '(', ')', ')', '(', ')', ')', '(', ')', '(', ')', '(', '(', '(', ')',
                ')', ')', ')', '(', ')', '(', ')', ')', ')', '(', '(', '(', '(', '(', '(', '(',
                ')', ')',
            ],
        ];

        dbg!(Solution::has_valid_path(grid));
    }

    #[test]
    fn example_8() {
        let grid: Vec<Vec<char>> = vec![vec!['(', ')']];

        assert!(Solution::has_valid_path(grid));
    }
}
