use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq)]
struct BingoBoard {
    numbers: [BoardSquare; 25],
}

impl BingoBoard {
    fn call_number(&mut self, number: u8) {
        for square in self.numbers.iter_mut() {
            if square.number == number {
                square.is_called = true;
            }
        }
    }

    fn wins(&self) -> bool {
        let rows = [
            self.numbers[0..5].iter(),
            self.numbers[5..10].iter(),
            self.numbers[10..15].iter(),
            self.numbers[15..20].iter(),
            self.numbers[20..25].iter(),
        ];
        for mut horizontal in rows {
            if horizontal.all(|square| square.is_called) {
                return true;
            }
        }
        let columns = [
            self.numbers.iter().step_by(5),
            self.numbers[1..].iter().step_by(5),
            self.numbers[2..].iter().step_by(5),
            self.numbers[3..].iter().step_by(5),
            self.numbers[4..].iter().step_by(5),
        ];
        for mut vertical in columns {
            if vertical.all(|square| square.is_called) {
                return true;
            }
        }
        false
    }
}

impl FromStr for BingoBoard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // a bingo board is a 5x5 grid of whitespace spearate ascii numbers
        let mut numbers = [BoardSquare::default(); 25];
        let mut i = 0;
        for str in s.trim().split_whitespace() {
            if i > 25 {
                return Err(anyhow!(
                    "invalid bingo board size. expected 25 spaces, got {}",
                    i
                ));
            }
            numbers[i] = BoardSquare {
                number: str.parse()?,
                is_called: false,
            };
            i += 1;
        }
        if i != 25 {
            return Err(anyhow!(
                "invalid bingo board size. expected 25 spaces, got {}",
                i
            ));
        }

        Ok(BingoBoard { numbers })
    }
}
impl std::fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, square) in self.numbers.iter().enumerate() {
            if i % 5 == 0 {
                writeln!(f)?;
            }
            write!(f, "{:?} ", square)?;
        }
        write!(f, "\n\n")
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct BoardSquare {
    number: u8,
    is_called: bool,
}
impl std::fmt::Debug for BoardSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_called {
            // bold self.number using terminal escape codes
            write!(f, "\x1B[1;31m{:2 }\x1B[0m", self.number)
        } else {
            write!(f, "{:2 }", self.number)
        }
    }
}
impl Default for BoardSquare {
    fn default() -> Self {
        BoardSquare {
            number: 0,
            is_called: false,
        }
    }
}

struct Part1Problem {
    numbers: Vec<u8>,
    bingo_boards: Vec<BingoBoard>,
}
impl FromStr for Part1Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        // first line is a list of numbers, comma separated, then a list of bingo boards separated by two newlines

        let (first_line, rest) = input
            .split_once("\n\n")
            .ok_or(anyhow!("Expected input to start with two newlines"))?;

        let numbers: Vec<u8> = first_line
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|_| anyhow!("could not parse number {} from first line of input", s))
            })
            .collect::<Result<Vec<u8>>>()?;

        let bingo_boards: Vec<BingoBoard> = rest
            .split("\n\n")
            .enumerate()
            .map(|(i, s)| {
                s.parse()
                    .with_context(|| format!("could not parse {}th bingo board in input {}", i, s))
            })
            .collect::<Result<Vec<BingoBoard>>>()?;

        Ok(Part1Problem {
            numbers,
            bingo_boards,
        })
    }
}
impl Part1Problem {
    fn get_first_winning_board_and_number(&mut self) -> Option<(BingoBoard, u8)> {
        for number in self.numbers.iter() {
            for bingo_board in self.bingo_boards.iter_mut() {
                bingo_board.call_number(*number);
                if bingo_board.wins() {
                    return Some((bingo_board.clone(), *number));
                }
            }
        }
        None
    }

    fn get_last_winning_board_and_number(&mut self) -> Option<(BingoBoard, u8)> {
        let mut active_boards = self.bingo_boards.clone();
        for number in self.numbers.iter() {
            let mut last_board = None;
            for bingo_board in active_boards.iter_mut() {
                bingo_board.call_number(*number);
                if bingo_board.wins() {
                    last_board = Some(bingo_board.clone());
                }
            }
            // this is awkward, what we really want is:
            // let last_board = active_boards.drain_filter(|b| b.wins()).last();
            // which only checks each board once for winning, and efficiently removes winning boards from the vector
            // but we can't use it because it's unstable, and I don't want these AoC solutions to bit rot
            active_boards = active_boards.into_iter().filter(|b| !b.wins()).collect();
            if active_boards.len() == 0 {
                if let Some(board) = last_board {
                    return Some((board, *number));
                } else {
                    panic!("internal error: no winning boards left, but we didn't find a final winning board either??");
                }
            }
        }
        None
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut problem = input.parse::<Part1Problem>()?;
    let (board, number) = problem
        .get_first_winning_board_and_number()
        .ok_or(anyhow!("no winning board"))?;

    let unmarked_squares_sum: u64 = board
        .numbers
        .iter()
        .filter(|square| !square.is_called)
        .map(|square| square.number as u64)
        .sum();

    Ok(number as u64 * unmarked_squares_sum)
}

fn part_2(input: &str) -> Result<u64> {
    let mut problem = input.parse::<Part1Problem>()?;
    let (board, number) = problem
        .get_last_winning_board_and_number()
        .ok_or(anyhow!("no final winning board??"))?;

    let unmarked_squares_sum: u64 = board
        .numbers
        .iter()
        .filter(|square| !square.is_called)
        .map(|square| square.number as u64)
        .sum();

    Ok(number as u64 * unmarked_squares_sum)
}

const EXAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[test]
fn test_part1() {
    assert_eq!(
        part_1(EXAMPLE_INPUT).context("Example input").unwrap(),
        4_512
    );
    assert_eq!(
        part_1(include_str!("./day4.txt"))
            .context("Real input")
            .unwrap(),
        38_594
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part_2(EXAMPLE_INPUT).context("Example input").unwrap(),
        1_924
    );
    assert_eq!(
        part_2(include_str!("./day4.txt"))
            .context("Real input")
            .unwrap(),
        21_184
    );
}
