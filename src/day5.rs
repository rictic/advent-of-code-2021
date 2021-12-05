use anyhow::{self, Context, Result};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    points: BTreeMap<Point, u64>,
    // Bounds so we can efficiently draw the board
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            points: BTreeMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }
}

impl Board {
    fn add_line(&mut self, line: LineSegment) {
        // update bounds
        self.min_x = self.min_x.min(line.start.x.min(line.end.x));
        self.max_x = self.max_x.max(line.start.x.max(line.end.x));
        self.min_y = self.min_y.min(line.start.y.min(line.end.y));
        self.max_y = self.max_y.max(line.start.y.max(line.end.y));

        // initially just supporting straight lines
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
            self.points
                .entry(Point { x, y })
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if x == line.end.x && y == line.end.y {
                break;
            }
            x += dx;
            y += dy;
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let count = *self.points.get(&Point { x, y }).unwrap_or(&0);
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

fn part_1(input: &str) -> Result<usize> {
    let mut board = Board::default();
    let lines = input
        .lines()
        .map(|line| line.parse::<LineSegment>().context("Part 1 input"))
        .collect::<Result<Vec<LineSegment>>>()?;
    for line in lines {
        if !line.is_straight() {
            continue;
        }
        board.add_line(line);
    }
    Ok(board.points.values().filter(|&count| *count > 1).count())
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
    let mut board = Board::default();
    let lines = input
        .lines()
        .map(|line| line.parse::<LineSegment>().context("Part 2 input"))
        .collect::<Result<Vec<LineSegment>>>()?;
    for line in lines {
        board.add_line(line);
    }
    Ok(board.points.values().filter(|&count| *count > 1).count())
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
}
