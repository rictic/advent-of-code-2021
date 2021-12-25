use anyhow::Result;
use smallvec::SmallVec;

fn neighbors(grid: &Vec<Vec<u8>>) -> Vec<(u8, (usize, usize), SmallVec<[u8; 4]>)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, &v)| {
                let mut neighbors = SmallVec::<[u8; 4]>::new();
                if y > 0 {
                    neighbors.push(grid[y - 1][x]);
                }
                if y < grid.len() - 1 {
                    neighbors.push(grid[y + 1][x]);
                }
                if x > 0 {
                    neighbors.push(grid[y][x - 1]);
                }
                if x < line.len() - 1 {
                    neighbors.push(grid[y][x + 1]);
                }
                Some((v, (y, x), neighbors))
            })
        })
        .collect()
}

fn neighbors_of(grid: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> SmallVec<[u8; 4]> {
    let mut neighbors = SmallVec::<[u8; 4]>::new();
    let line = &grid[y];
    if y > 0 {
        neighbors.push(grid[y - 1][x]);
    }
    if y < grid.len() - 1 {
        neighbors.push(grid[y + 1][x]);
    }
    if x > 0 {
        neighbors.push(grid[y][x - 1]);
    }
    if x < line.len() - 1 {
        neighbors.push(grid[y][x + 1]);
    }
    neighbors
}

fn minima(grid: &Vec<Vec<u8>>) -> Vec<(u8, (usize, usize))> {
    neighbors(grid)
        .into_iter()
        .filter_map(|(val, loc, neighbors)| {
            if neighbors.into_iter().all(|neighbor| neighbor > val) {
                Some((val, loc))
            } else {
                None
            }
        })
        .collect()
}

fn part_1(input: &str) -> Result<u64> {
    let grid = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(minima(&grid)
        .into_iter()
        .map(|(val, _)| 1 + val as u64)
        .sum())
}

#[test]
fn test_part_1() {
    let input = "
2199943210
3987894921
9856789892
8767896789
9899965678"
        .trim();
    assert_eq!(part_1(input).unwrap(), 15);
    assert_eq!(part_1(include_str!("./day9.txt")).unwrap(), 486);
}

fn basin_size(grid: &Vec<Vec<u8>>, start: (usize, usize)) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = vec![start];
    let mut size = 0;
    while let Some(loc) = queue.pop() {
        if visited[loc.0][loc.1] {
            continue;
        }
        visited[loc.0][loc.1] = true;
        let loc = (loc.0 as i64, loc.1 as i64);
        size += 1;
        for &neighbor in &[
            (loc.0 - 1, loc.1),
            (loc.0 + 1, loc.1),
            (loc.0, loc.1 - 1),
            (loc.0, loc.1 + 1),
        ] {
            if neighbor.0 >= 0
                && neighbor.0 < grid.len() as i64
                && neighbor.1 >= 0
                && neighbor.1 < grid[0].len() as i64
                && grid[neighbor.0 as usize][neighbor.1 as usize] != 9
            {
                queue.push((neighbor.0 as usize, neighbor.1 as usize));
            }
        }
    }
    size
}

fn part_2(input: &str) -> Result<u64> {
    let grid = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let minima = minima(&grid);
    let mut basin_sizes = minima
        .iter()
        .map(|(_, loc)| {
            let size = basin_size(&grid, *loc);
            size as u64
        })
        .collect::<Vec<_>>();
    basin_sizes.sort();

    Ok(basin_sizes.into_iter().rev().take(3).product())
}

#[test]
fn test_part_2() {
    let input = "
2199943210
3987894921
9856789892
8767896789
9899965678"
        .trim();

    assert_eq!(part_2(input).unwrap(), 1134);
    assert_eq!(part_2(include_str!("./day9.txt")).unwrap(), 1059300);
}
