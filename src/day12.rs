use anyhow::{Error, Result};
use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};

use smallvec::SmallVec;

#[derive(Default, Debug)]
struct Graph {
    names: HashMap<String, usize>,
    edges: Vec<(Size, SmallVec<[usize; 5]>)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Size {
    Big,
    Small,
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut graph = Graph::default();
        for line in s.trim().lines() {
            let (from, to) = line
                .split_once("-")
                .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
            let from = graph.add_name(from);
            let to = graph.add_name(to);
            graph.edges[from].1.push(to);
            graph.edges[to].1.push(from);
        }

        Ok(graph)
    }
}

impl Graph {
    fn add_name(&mut self, name: &str) -> usize {
        let size = if name.chars().next().unwrap().is_uppercase() {
            Size::Big
        } else {
            Size::Small
        };
        *self.names.entry(name.to_string()).or_insert_with(|| {
            self.edges.push((size, SmallVec::new()));
            self.edges.len() - 1
        })
    }

    fn count_paths(&self) -> Result<u64> {
        Ok(self.count_paths_from_to(
            *self.names.get("start").ok_or(anyhow::anyhow!("No start"))?,
            *self.names.get("end").ok_or(anyhow::anyhow!("No end"))?,
            &mut Default::default(),
            &mut Default::default(),
        ))
    }

    fn count_paths_from_to(
        &self,
        from: usize,
        to: usize,
        small_visited: &mut BTreeSet<usize>,
        path: &mut Vec<usize>,
    ) -> u64 {
        let mut count = 0;
        let is_small = self.edges[from].0 == Size::Small;
        if is_small {
            if small_visited.contains(&from) {
                return 0;
            }
            small_visited.insert(from);
        }
        path.push(from);
        for &neighbor in &self.edges[from].1 {
            if neighbor == to {
                count += 1;
            } else {
                count += self.count_paths_from_to(neighbor, to, small_visited, path);
            }
        }
        path.pop();
        if is_small {
            small_visited.remove(&from);
        }
        count
    }
}

fn part_1(input: &str) -> Result<u64> {
    let graph = input.parse::<Graph>()?;
    graph.count_paths()
}

#[test]
fn test_part_1() {
    assert_eq!(
        part_1(
            "
start-A
start-b
A-c
A-b
b-d
A-end
b-end"
        )
        .unwrap(),
        10
    );
    assert_eq!(part_1(include_str!("day12.txt")).unwrap(), 4749);
}
