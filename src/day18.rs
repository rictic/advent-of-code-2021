use std::fmt::{Display, Formatter};
use ExplosionProcess::*;
use Shockwave::*;

use anyhow::{anyhow, Context, Error, Result};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

type Value = i64;

#[must_use]
enum ExplosionProcess {
    ChildExploded(Value, Value),
    Shockwave(Shockwave),
    Handled,
}

enum Shockwave {
    RightThenLeftMost(Value),
    LeftMost(Value),
    LeftThenRightMost(Value),
    RightMost(Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailNumber {
    Regular(Value),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl std::str::FromStr for SnailNumber {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        // snail numbers look like [[6,4],2]
        fn parse_one(s: &str) -> Result<(SnailNumber, &str)> {
            if s.starts_with('[') {
                let (left, s) =
                    parse_one(&s[1..]).with_context(|| format!("left number in {}", s))?;
                if !s.starts_with(',') {
                    return Err(anyhow!("expected comma after left number in {}", s));
                }
                let (right, s) = parse_one(&s[1..]).with_context(|| {
                    format!(
                        "right number in {} after parsing left number to {}",
                        s, left
                    )
                })?;
                if !s.starts_with(']') {
                    return Err(anyhow!(
                        "expected right bracket after right number in {}",
                        s
                    ));
                }

                Ok((SnailNumber::Pair(Box::new(left), Box::new(right)), &s[1..]))
            } else {
                let idx = s
                    .find(|c| c == ',' || c == ']')
                    .ok_or(anyhow!("expected ',' or ']'"))?;
                let (num, rem) = (&s[0..idx], &s[idx..]);

                Ok((
                    SnailNumber::Regular(num.parse().with_context(|| {
                        format!(
                            "parsing {:?} got number {:?} and remainder {:?}",
                            s, num, rem
                        )
                    })?),
                    rem,
                ))
            }
        }
        let line = line.trim();
        let (snail_number, s) =
            parse_one(line).with_context(|| format!("Error parsing line {:?}", line))?;
        if !s.is_empty() {
            return Err(anyhow!("expected end of string, found {:?}", s));
        }
        Ok(snail_number)
    }
}
impl Display for SnailNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Regular(n) => write!(f, "{}", n),
            SnailNumber::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

impl SnailNumber {
    fn magnitude(&self) -> Value {
        match self {
            SnailNumber::Regular(n) => *n,
            SnailNumber::Pair(left, right) => (3 * left.magnitude()) + (2 * right.magnitude()),
        }
    }

    fn make_regular(&mut self) {
        loop {
            // println!("{}", self);
            match self.try_explode(0) {
                Some(ChildExploded(..)) => {
                    unreachable!("root node can't explode")
                }
                Some(Shockwave(..)) => {
                    continue;
                }
                Some(Handled) => {
                    continue;
                }
                None => {}
            }
            if self.try_split() {
                // println!("Split!");
                continue;
            }
            break;
        }
    }

    fn try_explode(&mut self, depth: usize) -> Option<ExplosionProcess> {
        if depth >= 4 {
            match self {
                SnailNumber::Regular(_) => {}
                SnailNumber::Pair(l, r) => {
                    let (l, r) = match (l.as_mut(), r.as_mut()) {
                        (SnailNumber::Regular(l), SnailNumber::Regular(r)) => (*l, *r),
                        _ => unreachable!("only pairs of regular numbers should explode"),
                    };
                    // println!("[{},{}] is too deep, exploding", l, r);
                    *self = SnailNumber::Regular(0);
                    return Some(ChildExploded(l, r));
                }
            }
        }
        // So many clones, how can we remove them?
        let (left, right) = match self {
            SnailNumber::Regular(_) => return None,
            SnailNumber::Pair(left, right) => (left, right),
        };

        match left.try_explode(depth + 1) {
            Some(ChildExploded(ll, lr)) => {
                right.add_to_leftmost_regular(lr);
                return Some(Shockwave(LeftThenRightMost(ll)));
            }
            Some(Shockwave(RightThenLeftMost(v))) => {
                right.add_to_leftmost_regular(v);
                return Some(Handled);
            }
            Some(Shockwave(RightMost(v))) => {
                right.add_to_rightmost_regular(v);
                return Some(Handled);
            }
            Some(v) => {
                return Some(v);
            }
            None => {}
        }
        match right.try_explode(depth + 1) {
            Some(ChildExploded(rl, rr)) => {
                left.add_to_rightmost_regular(rl);
                return Some(Shockwave(RightThenLeftMost(rr)));
            }
            Some(Shockwave(LeftThenRightMost(v))) => {
                left.add_to_rightmost_regular(v);
                return Some(Handled);
            }
            Some(Shockwave(LeftMost(v))) => {
                left.add_to_leftmost_regular(v);
                return Some(Handled);
            }
            Some(v) => {
                return Some(v);
            }
            None => {}
        }

        None
    }

    fn add_to_leftmost_regular(&mut self, value: Value) {
        match self {
            SnailNumber::Regular(v) => {
                // println!("propagating shockwave {} to {}", value, v);
                *v += value
            }
            SnailNumber::Pair(left, _) => {
                left.add_to_leftmost_regular(value);
            }
        }
    }

    fn add_to_rightmost_regular(&mut self, value: Value) {
        match self {
            SnailNumber::Regular(v) => {
                // println!("propagating shockwave {} to {}", value, v);
                *v += value
            }
            SnailNumber::Pair(_, right) => {
                right.add_to_rightmost_regular(value);
            }
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            SnailNumber::Regular(val) => {
                if *val > 9 {
                    let is_odd = *val % 2;
                    let halved = *val / 2;
                    let left = SnailNumber::Regular(halved);
                    let right = SnailNumber::Regular(halved + is_odd);
                    *self = SnailNumber::Pair(Box::new(left), Box::new(right));
                    true
                } else {
                    false
                }
            }
            SnailNumber::Pair(left, right) => left.try_split() || right.try_split(),
        }
    }

    fn add(&mut self, other: SnailNumber) {
        let self_copy = self.clone();
        *self = SnailNumber::Pair(Box::new(self_copy), Box::new(other));
    }
}

fn sum_lines(input: &str) -> Result<SnailNumber> {
    let number = input
        .trim()
        .lines()
        .map(|l| l.parse::<SnailNumber>())
        .reduce(|a, b| {
            let mut a = a?;
            a.add(b?);
            a.make_regular();
            Ok(a)
        })
        .ok_or(anyhow!("expected at least one line"))??;
    Ok(number)
}

fn regularize(input: &str) -> Result<String> {
    let mut number = input.parse::<SnailNumber>()?;
    number.make_regular();
    Ok(number.to_string())
}

fn part_1(input: &str) -> Result<Value> {
    Ok(sum_lines(input)?.magnitude())
}

#[test]
fn test_part_1() {
    assert_eq!(
        regularize("[[[[[9,8],1],2],3],4]").unwrap(),
        "[[[[0,9],2],3],4]"
    );
    assert_eq!(
        sum_lines(
            "
    [1,1]
    [2,2]
    [3,3]
    [4,4]"
        )
        .unwrap()
        .to_string(),
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    );
    assert_eq!(
        sum_lines(
            "
    [1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
            "
        )
        .unwrap()
        .to_string(),
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
    );
    let input = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    assert_eq!(
        sum_lines(input).unwrap().to_string(),
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
    );
    assert_eq!(part_1(input).unwrap(), 4140);
    assert_eq!(part_1(include_str!("day18.txt")).unwrap(), 3869);
}

fn part_2(input: &str) -> Result<Value> {
    let numbers = input
        .trim()
        .lines()
        .map(|l| l.parse::<SnailNumber>())
        .collect::<Result<Vec<_>>>()?;
    numbers
        .par_iter()
        .enumerate()
        .filter_map(|(l_idx, left)| {
            numbers
                .par_iter()
                .enumerate()
                .filter_map(move |(r_idx, right)| {
                    if l_idx == r_idx {
                        return None;
                    }
                    let mut left = left.clone();
                    left.add(right.clone());
                    left.make_regular();
                    Some(left.magnitude())
                })
                .max()
        })
        .max()
        .ok_or(anyhow!("expected at least two lines"))
}

#[test]
fn test_part_2() {
    let input = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    assert_eq!(part_2(input).unwrap(), 3993);
    assert_eq!(part_2(include_str!("day18.txt")).unwrap(), 4671);
}
