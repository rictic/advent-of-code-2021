use anyhow::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BracketKind {
    Curly,
    Square,
    Angle,
    Paren,
}
impl BracketKind {
    fn syntax_score(self) -> u64 {
        match self {
            BracketKind::Paren => 3,
            BracketKind::Square => 57,
            BracketKind::Curly => 1197,
            BracketKind::Angle => 25137,
        }
    }

    fn autocomplete_score(self) -> u64 {
        match self {
            BracketKind::Paren => 1,
            BracketKind::Square => 2,
            BracketKind::Curly => 3,
            BracketKind::Angle => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bracket {
    Open(BracketKind),
    Close(BracketKind),
}
impl TryFrom<char> for Bracket {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '{' => Ok(Bracket::Open(BracketKind::Curly)),
            '}' => Ok(Bracket::Close(BracketKind::Curly)),
            '[' => Ok(Bracket::Open(BracketKind::Square)),
            ']' => Ok(Bracket::Close(BracketKind::Square)),
            '<' => Ok(Bracket::Open(BracketKind::Angle)),
            '>' => Ok(Bracket::Close(BracketKind::Angle)),
            '(' => Ok(Bracket::Open(BracketKind::Paren)),
            ')' => Ok(Bracket::Close(BracketKind::Paren)),
            _ => Err(anyhow::anyhow!("Invalid bracket: {}", value)),
        }
    }
}

enum LineStatus {
    Complete,
    Corrupt(BracketKind),
    Incomplete(Vec<BracketKind>),
}

fn evaluate_line(brackets: impl Iterator<Item = Bracket>) -> LineStatus {
    let mut stack = Vec::new();
    for bracket in brackets {
        match bracket {
            Bracket::Open(kind) => {
                stack.push(kind);
            }
            Bracket::Close(kind) => {
                if let Some(expected) = stack.pop() {
                    if kind != expected {
                        return LineStatus::Corrupt(kind);
                    }
                } else {
                    return LineStatus::Corrupt(kind);
                }
            }
        }
    }
    if stack.is_empty() {
        LineStatus::Complete
    } else {
        LineStatus::Incomplete(stack)
    }
}

fn get_corruption_char(brackets: impl Iterator<Item = Bracket>) -> Option<BracketKind> {
    match evaluate_line(brackets) {
        LineStatus::Corrupt(kind) => Some(kind),
        _ => None,
    }
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            match get_corruption_char(line.chars().map(Bracket::try_from).map(Result::unwrap)) {
                Some(kind) => kind.syntax_score(),
                None => 0,
            }
        })
        .sum()
}

#[test]
fn test_part_1() {
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .trim();
    assert_eq!(part_1(input), 26_397);
    assert_eq!(part_1(include_str!("day10.txt")), 311895);
}

fn part_2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .filter_map(|line| {
            let completions =
                match evaluate_line(line.chars().map(Bracket::try_from).map(Result::unwrap)) {
                    LineStatus::Incomplete(stack) => stack,
                    _ => return None,
                };
            Some(
                completions
                    .into_iter()
                    .rev()
                    .fold(0u64, |acc, kind| (acc * 5) + kind.autocomplete_score()),
            )
        })
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn test_part_2() {
    let input = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .trim();
    assert_eq!(part_2(input), 288_957);
    assert_eq!(part_2(include_str!("day10.txt")), 2_904_180_541);
}
