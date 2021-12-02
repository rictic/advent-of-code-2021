use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    steps: i64,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse like "forward 10" or "down 20"
        let mut parts = s.split_whitespace();
        let direction = match parts.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => return Err(format!("Invalid direction: {}", s)),
        };
        let steps = parts
            .next()
            .ok_or("Expected direction then number")?
            .parse::<i64>()
            .map_err(|e| format!("Invalid number: {}", e))?;
        if parts.next().is_some() {
            return Err(format!("Too many parts: {}", s));
        }

        Ok(Command { direction, steps })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Location {
    depth: i64,
    x: i64,
}
impl Location {
    fn new() -> Self {
        Location { depth: 0, x: 0 }
    }
    fn take_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => self.x += command.steps,
            Direction::Down => self.depth += command.steps,
            Direction::Up => self.depth -= command.steps,
        }
    }
}

fn take_commands(commands: &str) -> Result<Location, String> {
    let mut location = Location::new();
    for command in commands.lines() {
        let command = Command::from_str(command)?;
        location.take_command(&command);
    }
    Ok(location)
}

fn part_1(input: &str) -> Result<i64, String> {
    let location = take_commands(input)?;
    Ok(location.x.abs() * location.depth.abs())
}

#[test]
fn test_part1() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
    assert_eq!(take_commands(input).unwrap(), Location { depth: 10, x: 15 });
    assert_eq!(part_1(input).unwrap(), 150);

    assert_eq!(part_1(include_str!("./day2.txt")), Ok(1_561_344));
}

#[derive(Debug, PartialEq, Eq)]
struct TrickyLocation {
    depth: i64,
    x: i64,
    aim: i64,
}
impl TrickyLocation {
    fn new() -> Self {
        TrickyLocation {
            depth: 0,
            x: 0,
            aim: 0,
        }
    }

    fn take_command(&mut self, command: &Command) {
        match command.direction {
            Direction::Down => self.aim += command.steps,
            Direction::Up => self.aim -= command.steps,
            Direction::Forward => {
                self.x += command.steps;
                self.depth += self.aim * command.steps;
            }
        }
    }
}

fn take_commands_tricky(commands: &str) -> Result<Location, String> {
    let mut location = TrickyLocation::new();
    for command in commands.lines() {
        let command = Command::from_str(command)?;
        location.take_command(&command);
    }
    Ok(Location {
        depth: location.depth,
        x: location.x,
    })
}

fn part_2(input: &str) -> Result<i64, String> {
    let location = take_commands_tricky(input)?;
    Ok(location.x.abs() * location.depth.abs())
}

#[test]
fn test_part2() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
    assert_eq!(
        take_commands_tricky(input).unwrap(),
        Location { depth: 60, x: 15 }
    );
    assert_eq!(part_2(input).unwrap(), 900);

    assert_eq!(part_2(include_str!("./day2.txt")), Ok(1_848_454_425));
}
