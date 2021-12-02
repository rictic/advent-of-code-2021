use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

fn take_commands<Loc>(commands: &str) -> Result<SimpleLocation, String>
where
    Loc: Location + Into<SimpleLocation> + Default,
{
    let mut location = Loc::default();
    for command in commands.lines() {
        location.take_command(Command::from_str(command)?);
    }
    Ok(location.into())
}

trait Location {
    fn take_command(&mut self, command: Command);
}

#[derive(Debug, PartialEq, Eq, Default)]
struct SimpleLocation {
    depth: i64,
    x: i64,
}
impl Location for SimpleLocation {
    fn take_command(&mut self, command: Command) {
        match command.direction {
            Direction::Forward => self.x += command.steps,
            Direction::Down => self.depth += command.steps,
            Direction::Up => self.depth -= command.steps,
        }
    }
}

fn part_1(input: &str) -> Result<i64, String> {
    let location = take_commands::<SimpleLocation>(input)?;
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
    assert_eq!(
        take_commands::<SimpleLocation>(input).unwrap(),
        SimpleLocation { depth: 10, x: 15 }
    );
    assert_eq!(part_1(input).unwrap(), 150);

    assert_eq!(part_1(include_str!("./day2.txt")), Ok(1_561_344));
}

#[derive(Debug, PartialEq, Eq, Default)]
struct TrickyLocation {
    depth: i64,
    x: i64,
    aim: i64,
}
impl Location for TrickyLocation {
    fn take_command(&mut self, command: Command) {
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
impl Into<SimpleLocation> for TrickyLocation {
    fn into(self) -> SimpleLocation {
        SimpleLocation {
            depth: self.depth,
            x: self.x,
        }
    }
}

fn part_2(input: &str) -> Result<i64, String> {
    let location = take_commands::<TrickyLocation>(input)?;
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
        take_commands::<TrickyLocation>(input).unwrap(),
        SimpleLocation { depth: 60, x: 15 }
    );
    assert_eq!(part_2(input).unwrap(), 900);

    assert_eq!(part_2(include_str!("./day2.txt")), Ok(1_848_454_425));
}
