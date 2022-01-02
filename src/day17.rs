use std::{collections::HashSet, fmt::Display};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use regex::Regex;
use Effect::*;
use Rel::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rel {
    Before,
    After,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RelativePosition(Rel, Rel);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Effect {
    Hit,
    TooShort,
    DroppingTooFast,
    TooLong,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    top: i64,
    left: i64,
    bottom: i64,
    right: i64,
}
impl std::str::FromStr for Bounds {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // Specified like "target area: x=20..30, y=-10..-5
        let re = Regex::new(
            r"target area: x=(?P<left>-?\d+)..(?P<right>-?\d+), y=(?P<bottom>-?\d+)..(?P<top>-?\d+)",
        )?;
        let caps = re
            .captures(s)
            .ok_or(anyhow!("Invalid target area: {}", s))?;
        Ok(Bounds {
            top: caps["top"].parse()?,
            left: caps["left"].parse()?,
            bottom: caps["bottom"].parse()?,
            right: caps["right"].parse()?,
        })
    }
}

impl Bounds {
    fn compare(self, point: Point) -> RelativePosition {
        let x = if point.x < self.left {
            Before
        } else if point.x > self.right {
            After
        } else {
            Inside
        };
        let y = if point.y < self.bottom {
            Before
        } else if point.y > self.top {
            After
        } else {
            Inside
        };
        RelativePosition(x, y)
    }

    fn fire_at(self, (dx, dy): (i64, i64)) -> Effect {
        Shot::new(self, dx, dy).fire()
    }

    fn include(&mut self, point: Point) {
        self.top = self.top.max(point.y);
        self.left = self.left.min(point.x);
        self.bottom = self.bottom.min(point.y);
        self.right = self.right.max(point.x);
    }

    fn plausible_initial_velocities(self) -> impl Iterator<Item = (i64, i64)> {
        let target_height = self.top - self.bottom;
        let dys = self.bottom..target_height * 10;
        let dxs = 0..self.right * 2;
        dxs.cartesian_product(dys)
    }

    fn shots_that_hit(self) -> impl Iterator<Item = Shot> {
        self.plausible_initial_velocities()
            .filter_map(move |(dx, dy)| {
                let shot = Shot::new(self, dx, dy);
                if shot.fire() == Hit {
                    Some(shot)
                } else {
                    None
                }
            })
    }

    fn max_height_hit(self) -> Option<i64> {
        self.shots_that_hit()
            .filter_map(|shot| shot.map(|p| p.y).max())
            .max()
    }
}

impl IntoIterator for Bounds {
    type Item = Point;

    type IntoIter = BoundsIter;

    fn into_iter(self) -> Self::IntoIter {
        BoundsIter {
            bounds: self,
            x: self.left,
            y: self.top,
        }
    }
}
struct BoundsIter {
    bounds: Bounds,
    x: i64,
    y: i64,
}
impl Iterator for BoundsIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.bounds.right {
            self.x = self.bounds.left;
            self.y -= 1;
        }
        if self.y < self.bounds.bottom {
            return None;
        }
        let ret = Point {
            x: self.x,
            y: self.y,
        };
        self.x += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone, Copy)]
struct Shot {
    target_area: Bounds,
    dx: i64,
    dy: i64,
    x: i64,
    y: i64,
    done: bool,
}
impl Shot {
    fn new(target_area: Bounds, dx: i64, dy: i64) -> Self {
        Self {
            target_area,
            dx,
            dy,
            x: 0,
            y: 0,
            done: false,
        }
    }
    fn fire(self) -> Effect {
        let mut prev_rel = self.target_area.compare(Point { x: 0, y: 0 });
        let mut prev = Point { x: 0, y: 0 };
        for pos in self {
            let rel = self.target_area.compare(pos);
            if rel == RelativePosition(Inside, Inside) {
                return Hit;
            }
            if prev.x == pos.x {
                match rel {
                    RelativePosition(Before, _) => {
                        return TooShort;
                    }
                    RelativePosition(After, _) => {
                        return TooLong;
                    }
                    _ => {}
                }
            }
            match (prev_rel, rel) {
                (RelativePosition(Before, _), RelativePosition(After, _)) => {
                    return TooLong;
                }
                (RelativePosition(_, Before), RelativePosition(_, After)) => {
                    return DroppingTooFast;
                }
                _ => {}
            }
            prev_rel = rel;
            prev = pos;
        }
        DroppingTooFast
    }
}
impl Iterator for Shot {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        self.x += self.dx;
        self.y += self.dy;
        self.dy -= 1;
        // move dx one step closer to zero
        match self.dx.cmp(&0) {
            std::cmp::Ordering::Less => self.dx += 1,
            std::cmp::Ordering::Greater => self.dx -= 1,
            _ => (),
        }
        if self.y < self.target_area.bottom {
            self.done = true;
        }
        Some(Point {
            x: self.x,
            y: self.y,
        })
    }
}
impl Display for Shot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.collect::<HashSet<_>>();
        let mut bounds = self.target_area;
        bounds.include(Point { x: 0, y: 0 });
        for point in points.iter() {
            bounds.include(*point);
        }
        for p in bounds {
            if p.x == bounds.left && p.y != bounds.top {
                write!(f, "\n")?;
            }
            if p.x == 0 && p.y == 0 {
                write!(f, "S")?;
            } else if points.contains(&p) {
                write!(f, "#")?;
            } else if self.target_area.compare(p) == RelativePosition(Inside, Inside) {
                write!(f, "T")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

fn part_1(input: &str) -> Result<i64> {
    let target_area = input.parse::<Bounds>()?;
    target_area.max_height_hit().ok_or(anyhow!("no hit"))
}

#[test]
fn test_part_1() {
    let input = "target area: x=20..30, y=-10..-5";
    let target_area = input.parse::<Bounds>().unwrap();
    assert_eq!(target_area.fire_at((7, 2)), Hit);
    assert_eq!(target_area.fire_at((6, 3)), Hit);
    assert_eq!(target_area.fire_at((9, 0)), Hit);
    assert_eq!(target_area.fire_at((17, -4)), TooLong);
    assert_eq!(target_area.fire_at((1, 10)), TooShort);
    assert_eq!(target_area.fire_at((100, 10)), TooLong);
    assert_eq!(part_1(input).unwrap(), 45);
    assert_eq!(part_1(include_str!("day17.txt")).unwrap(), 23005);
}

fn part_2(input: &str) -> Result<usize> {
    let target_area = input.parse::<Bounds>()?;
    Ok(target_area.shots_that_hit().count())
}

#[test]
fn test_part_2() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(part_2(input).unwrap(), 112);
    assert_eq!(part_2(include_str!("day17.txt")).unwrap(), 2040);
}
