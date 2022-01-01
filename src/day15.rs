use anyhow::{anyhow, Error, Result};
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    str::FromStr,
};

use smallvec::SmallVec;
struct Cavern {
    costs: Vec<Vec<u8>>,
}

impl FromStr for Cavern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut costs = Vec::new();
        for line in s.lines() {
            costs.push(
                line.chars()
                    .map(|c| Ok(c.to_digit(10).ok_or(anyhow!("Invalid cost: {}", c))? as u8))
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        Ok(Cavern { costs })
    }
}

impl Cavern {
    fn get_cost(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || x >= self.costs.len() as i32 || y < 0 || y >= self.costs[0].len() as i32 {
            return None;
        }
        Some(self.costs[y as usize][x as usize])
    }
    fn neighbors(&self, x: i32, y: i32) -> SmallVec<[(u8, i32, i32); 4]> {
        let mut neighbors = SmallVec::new();
        if y > 0 {
            let y = y - 1;
            neighbors.push((self.costs[(y) as usize][x as usize], x, y));
        }
        if y < self.costs.len() as i32 - 1 {
            let y = y + 1;
            neighbors.push((self.costs[y as usize][x as usize], x, y));
        }
        if x > 0 {
            let x = x - 1;
            neighbors.push((self.costs[y as usize][x as usize], x, y));
        }
        if x < self.costs[0].len() as i32 - 1 {
            let x = x + 1;
            neighbors.push((self.costs[y as usize][x as usize], x, y));
        }
        neighbors
    }
    fn astar_search(&self) -> i32 {
        // Do an A* search from the top left to the bottom right to find the
        // minimum cost path.
        let mut open = BinaryHeap::new();
        let end = (self.costs[0].len() as i32 - 1) + (self.costs.len() as i32 - 1);
        open.push((std::cmp::Reverse(end), 0, 0, 0));
        let mut closed = HashSet::new();
        while let Some((_, cost, x, y)) = open.pop() {
            if x == self.costs.len() as i32 - 1 && y == self.costs[0].len() as i32 - 1 {
                return cost;
            }
            closed.insert((x, y));
            for &(neighbor_cost, neighbor_x, neighbor_y) in self.neighbors(x, y).iter() {
                if closed.contains(&(neighbor_x, neighbor_y)) {
                    continue;
                }
                let new_cost = cost + neighbor_cost as i32;
                let distance_from_goal = end - (neighbor_x + neighbor_y);
                open.push((
                    std::cmp::Reverse(distance_from_goal + new_cost),
                    new_cost,
                    neighbor_x,
                    neighbor_y,
                ));
            }
        }
        i32::MAX
    }
    fn expand(&self) -> Cavern {
        let mut new_costs = Vec::with_capacity(self.costs.len() * 5);
        for j in 0..5 {
            for row in &self.costs {
                let mut new_row = Vec::with_capacity(row.len() * 5);
                for i in 0..5 {
                    for val in row.iter() {
                        let mut new_val = val + i + j;
                        while new_val > 9 {
                            new_val -= 9;
                        }
                        new_row.push(new_val);
                    }
                }
                new_costs.push(new_row);
            }
        }
        Cavern { costs: new_costs }
    }
}

impl Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.costs {
            for val in row {
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_1(input: &str) -> Result<i32> {
    let cavern = Cavern::from_str(input)?;
    Ok(cavern.astar_search())
}

#[test]
fn test_part_1() {
    let input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
    "
    .trim();
    assert_eq!(part_1(input).unwrap(), 40);
    if cfg!(debug_assertions) {
        return; // skip test in debug mode, it's slow when it's unoptimized
    }
    assert_eq!(part_1(include_str!("day15.txt")).unwrap(), 390);
}

fn part_2(input: &str) -> Result<i32> {
    let cavern = Cavern::from_str(input)?.expand();
    Ok(cavern.astar_search())
}

#[test]
fn test_part_2() {
    let input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
    "
    .trim();
    assert_eq!(part_2(input).unwrap(), 315);
    if cfg!(debug_assertions) {
        return; // skip test in debug mode, it's slow when it's unoptimized
    }
    assert_eq!(part_2(include_str!("day15.txt")).unwrap(), 2814);
}
