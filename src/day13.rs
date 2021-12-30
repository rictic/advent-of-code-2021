use std::{collections::BTreeSet, str::FromStr};

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, Default)]
struct Grid {
    grid: BTreeSet<(i64, i64)>,
    xmax: i64,
    ymax: i64,
}

#[derive(Debug, Clone)]
struct ProblemInput {
    grid: Grid,
    folds: Vec<Fold>,
}
impl FromStr for ProblemInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (points, folds) = s.trim().split_once("\n\n").unwrap();
        let grid = points
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Invalid line: {}", line))?;
                let x = x.parse().context("x")?;
                let y = y.parse().context("y")?;
                Ok((x, y))
            })
            .fold(
                Ok(Grid::default()),
                |grid: Result<Grid>, point: Result<(i64, i64)>| {
                    let point = point?;
                    let mut grid = grid?;
                    grid.add_point(point.0, point.1);
                    Ok(grid)
                },
            )?;
        let folds = folds
            .lines()
            .map(Fold::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(ProblemInput { grid, folds })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Fold {
    AlongY(i64),
    AlongX(i64),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (fold_along, idx) = s
            .split_once("=")
            .ok_or_else(|| anyhow!("Invalid fold: {}", s))?;
        let idx = idx.parse().context("fold index")?;
        match fold_along {
            "fold along x" => Ok(Fold::AlongX(idx)),
            "fold along y" => Ok(Fold::AlongY(idx)),
            _ => Err(anyhow!("Invalid fold: {}", s)),
        }
    }
}

impl Grid {
    fn add_point(&mut self, x: i64, y: i64) {
        self.grid.insert((x, y));
        self.xmax = std::cmp::max(self.xmax, x);
        self.ymax = std::cmp::max(self.ymax, y);
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::AlongY(fold) => {
                let folded_points = self
                    .grid
                    .iter()
                    .cloned()
                    .filter(|(_x, y)| *y > fold)
                    .collect::<Vec<_>>();
                for (x, y) in folded_points.into_iter() {
                    self.grid.remove(&(x, y));
                    let distance_from_fold = y - fold;
                    self.grid.insert((x, fold - distance_from_fold));
                }
                self.ymax = fold - 1;
            }
            Fold::AlongX(fold) => {
                let folded_points = self
                    .grid
                    .iter()
                    .cloned()
                    .filter(|(x, _y)| *x > fold)
                    .collect::<Vec<_>>();
                for (x, y) in folded_points.into_iter() {
                    self.grid.remove(&(x, y));
                    let distance_from_fold = x - fold;
                    self.grid.insert((fold - distance_from_fold, y));
                }
                self.xmax = fold - 1;
            }
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.ymax {
            for x in 0..=self.xmax {
                if self.grid.contains(&(x, y)) {
                    write!(f, "#")?;
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
    let problem: ProblemInput = input.parse()?;
    let mut grid = problem.grid;
    grid.fold(problem.folds[0]);
    let result = grid.grid.len();
    grid.fold(problem.folds[1]);
    Ok(result)
}

fn part_2(input: &str) -> Result<String> {
    let problem: ProblemInput = input.parse()?;
    let mut grid = problem.grid;
    for fold in problem.folds {
        grid.fold(fold);
    }
    Ok(format!("{}", grid))
}

#[test]
fn test_part_1() {
    let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
    "#;
    assert_eq!(part_1(input).unwrap(), 17);
    assert_eq!(part_1(include_str!("day13.txt")).unwrap(), 653);
}

#[test]
fn test_part_2() {
    let input = r#"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
        "#;
    assert_eq!(
        part_2(input).unwrap().trim(),
        "
#####
#...#
#...#
#...#
#####
.....
....."
            .trim()
    );
    assert_eq!(
        part_2(include_str!("day13.txt")).unwrap().trim(),
        "
#....#..#.###..####.###..###..###..#..#.
#....#.#..#..#.#....#..#.#..#.#..#.#.#..
#....##...#..#.###..###..#..#.#..#.##...
#....#.#..###..#....#..#.###..###..#.#..
#....#.#..#.#..#....#..#.#....#.#..#.#..
####.#..#.#..#.####.###..#....#..#.#..#."
            .trim()
    );
}
