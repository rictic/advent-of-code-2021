use anyhow::anyhow;

#[derive(Copy, Clone, PartialEq, Eq)]
enum EnergyLevel {
    Value(u8),
    Flashed,
}

struct Grid([[EnergyLevel; 10]; 10]);

impl Grid {}

impl std::str::FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let mut grid = [[EnergyLevel::Value(0); 10]; 10];
        let mut i = 0;
        for ch in s.trim().chars() {
            if ch == '\n' {
                continue;
            }
            if i > 100 {
                return Err(anyhow!(
                    "invalid grid size. got more than 100 spaces in grid"
                ));
            }
            let value = ch
                .to_digit(10)
                .ok_or_else(|| anyhow!("invalid grid value: {}", ch))?;
            grid[i / 10][i % 10] = EnergyLevel::Value(value as u8);
            i += 1;
        }
        Ok(Grid(grid))
    }
}

impl Grid {
    fn step(&mut self) -> u64 {
        // first the energy level of each octopus is increased by one
        for row in self.0.iter_mut() {
            for square in row.iter_mut() {
                match square {
                    EnergyLevel::Value(value) => *value += 1,
                    EnergyLevel::Flashed => {}
                }
            }
        }
        // then any octopus with energy > 9 flashes
        for y in 0..10i32 {
            for x in 0..10 {
                {
                    let square = &mut self.0[y as usize][x as usize];
                    match square {
                        EnergyLevel::Value(value) if *value > 9 => *square = EnergyLevel::Flashed,
                        _ => continue,
                    }
                }
                // and each neighboring octopus (including diagonals)
                // also increases by one, and potentially flashes
                self.flash_at(x, y);
            }
        }
        // count the number of flashes and reset their values back to zero
        let mut flashes = 0;
        for row in self.0.iter_mut() {
            for square in row.iter_mut() {
                match square {
                    EnergyLevel::Flashed => {
                        flashes += 1;
                        *square = EnergyLevel::Value(0);
                    }
                    EnergyLevel::Value(_) => {}
                }
            }
        }
        flashes
    }

    fn flash_at(&mut self, x: i32, y: i32) {
        for (dy, dx) in &[
            (-1i32, 0i32),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ] {
            if let Some(neighbor) = self.get_square(x + dx, y + dy) {
                match neighbor {
                    EnergyLevel::Value(value) => {
                        *value += 1;
                        if *value > 9 {
                            *neighbor = EnergyLevel::Flashed;
                        } else {
                            continue;
                        }
                    }
                    EnergyLevel::Flashed => continue,
                }
                self.flash_at(x + dx, y + dy);
            }
        }
    }

    fn get_square(&mut self, x: i32, y: i32) -> Option<&mut EnergyLevel> {
        if x < 0 || x >= 10 || y < 0 || y >= 10 {
            return None;
        }
        Some(&mut self.0[y as usize][x as usize])
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for square in row.iter() {
                match square {
                    EnergyLevel::Value(value) if *value == 0 => {
                        write!(f, "\x1B[1;31m{}\x1B[0m", value)?
                    }
                    EnergyLevel::Flashed => write!(f, "\x1B[1;31mF\x1B[0m")?,
                    EnergyLevel::Value(value) => write!(f, "{}", value)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let mut grid: Grid = input.parse()?;
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += grid.step();
    }
    Ok(flashes)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let mut grid: Grid = input.parse()?;
    for i in 1.. {
        if grid.step() == 100 {
            return Ok(i);
        }
    }
    Err(anyhow!(
        "took more than a u64 number of steps to synchronize"
    ))
}

#[test]
fn test_part_1() {
    assert_eq!(
        part_1(
            "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
        )
        .unwrap(),
        1656
    );
    assert_eq!(part_1(include_str!("./day11.txt")).unwrap(), 1634);
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(
            "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
        )
        .unwrap(),
        195
    );
    assert_eq!(part_2(include_str!("./day11.txt")).unwrap(), 210);
}
