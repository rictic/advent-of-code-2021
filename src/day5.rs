use anyhow::{self, Context, Result};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    str::ParallelString,
};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid point: {:?}", s))?;
        let x = x.parse().context("x")?;
        let y = y.parse().context("y")?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LineSegment {
    start: Point,
    end: Point,
}
impl FromStr for LineSegment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow::anyhow!("Invalid line segment: {}", s))?;
        let start = start.parse::<Point>().context("start")?;
        let end = end.parse::<Point>().context("end")?;
        Ok(LineSegment { start, end })
    }
}
impl LineSegment {
    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

struct Board {
    // Counts of the number of times each point is touched by a line.
    points: Vec<u8>,
    // Bounds so we can map points to indexes
    bounds: Bounds,
}

impl Board {
    fn from_bounds(bounds: Bounds) -> Board {
        Board {
            points: vec![0; bounds.area()],
            bounds,
        }
    }

    fn add_line(&mut self, line: LineSegment) {
        // insert all points in the line
        let (mut x, mut y) = (line.start.x, line.start.y);
        let (mut dx, mut dy) = (line.end.x - line.start.x, line.end.y - line.start.y);
        if dx > 0 {
            dx = 1;
        } else if dx < 0 {
            dx = -1;
        }
        if dy > 0 {
            dy = 1;
        } else if dy < 0 {
            dy = -1;
        }
        loop {
            self.points[self.bounds.index(x, y)] =
                self.points[self.bounds.index(x, y)].saturating_add(1);
            if x == line.end.x && y == line.end.y {
                break;
            }
            x += dx;
            y += dy;
        }
    }

    fn combine(&mut self, other: Board) {
        for (mine, theirs) in self.points.iter_mut().zip(other.points.into_iter()) {
            *mine = mine.saturating_add(theirs);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.min_y..=self.bounds.max_y {
            for x in self.bounds.min_x..=self.bounds.max_x {
                let index = self.bounds.index(x, y);
                let count = self.points[index];
                if count > 10 {
                    write!(f, "X")?;
                } else if count > 0 {
                    write!(f, "{}", count)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Bounds {
    fn area(&self) -> usize {
        (self.max_x - self.min_x + 1) as usize * (self.max_y - self.min_y + 1) as usize
    }

    fn index(&self, x: i64, y: i64) -> usize {
        (x - self.min_x) as usize * (self.max_y - self.min_y + 1) as usize
            + (y - self.min_y) as usize
    }
}

fn get_bounds(lines: &[LineSegment]) -> Bounds {
    let mut bounds = Bounds {
        min_x: 0,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };
    for line in lines {
        bounds.min_x = bounds.min_x.min(line.start.x.min(line.end.x));
        bounds.max_x = bounds.max_x.max(line.start.x.max(line.end.x));
        bounds.min_y = bounds.min_y.min(line.start.y.min(line.end.y));
        bounds.max_y = bounds.max_y.max(line.start.y.max(line.end.y));
    }
    bounds
}

fn part_1(input: &str) -> Result<usize> {
    let line_segments = input
        .par_split('\n')
        // parse the line segments
        .map(|line| line.parse::<LineSegment>().context("Part 1 input"))
        .collect::<Result<Vec<_>>>()?;

    let bounds = get_bounds(&line_segments);

    let board = line_segments
        .into_par_iter()
        // group the segments into chunks and combine those chunks into boards
        .fold(
            || Board::from_bounds(bounds),
            |board, line| {
                let (line, mut board) = (line, board);
                if line.is_straight() {
                    board.add_line(line);
                }
                board
            },
        )
        // combine those boards down into one
        .reduce(
            || Board::from_bounds(bounds),
            |mut l, r| {
                l.combine(r);
                l
            },
        );
    let count_at_least_two = board.points.into_par_iter().filter(|&i| i > 1).count();
    Ok(count_at_least_two)
}

#[test]
fn test_part_1() {
    let input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .trim();
    assert_eq!(part_1(input).unwrap(), 5);
    assert_eq!(part_1(include_str!("./day5.txt")).unwrap(), 6_267);
}

fn part_2(input: &str) -> Result<usize> {
    let line_segments = input
        .par_split('\n')
        // parse the line segments
        .map(|line| line.parse::<LineSegment>().context("Part 2 input"))
        .collect::<Result<Vec<_>>>()?;

    let bounds = get_bounds(&line_segments);

    let board = line_segments
        .into_par_iter()
        // group the segments into chunks and combine those chunks into boards
        .fold(
            || Board::from_bounds(bounds),
            |board, line| {
                let (line, mut board) = (line, board);
                board.add_line(line);
                board
            },
        )
        // combine those boards down into one
        .reduce(
            || Board::from_bounds(bounds),
            |mut l, r| {
                l.combine(r);
                l
            },
        );
    let count_at_least_two = board.points.into_par_iter().filter(|&i| i > 1).count();
    Ok(count_at_least_two)
}

#[test]
fn test_part_2() {
    let input = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .trim();
    assert_eq!(part_2(input).unwrap(), 12);
    assert_eq!(part_2(include_str!("./day5.txt")).unwrap(), 20_196);

    // a huge input, such that it's actually faster to do it in parallel
    // let mut big_input_seed = String::from(include_str!("./day5.txt"));
    // big_input_seed.push_str("\n");
    // let big_input = big_input_seed.repeat(10_000);
    // assert_eq!(part_2(&big_input.trim()).unwrap(), 168274);
}
