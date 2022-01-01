use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

struct Problem {
    polymer: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (polymer, rules) = s.trim().split_once("\n\n").unwrap();
        let polymer = polymer.bytes().collect::<Vec<_>>();
        let rules = rules
            .lines()
            .map(|line| {
                let (pattern, result) = line
                    .split_once(" -> ")
                    .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
                let mut pattern = pattern.bytes();
                let (l, r) = (
                    pattern
                        .next()
                        .ok_or(anyhow::anyhow!("Invalid line: {}", line))?,
                    pattern
                        .next()
                        .ok_or(anyhow::anyhow!("Invalid line: {}", line))?,
                );
                if pattern.next().is_some() {
                    return Err(anyhow::anyhow!("Invalid line: {}", line));
                }
                let result = result
                    .bytes()
                    .next()
                    .ok_or(anyhow::anyhow!("Invalid line: {}", line))?;
                Ok(((l, r), result))
            })
            .collect::<anyhow::Result<HashMap<_, _>>>()?;
        Ok(Problem { polymer, rules })
    }
}

impl Problem {
    fn counts_after(&self, num_steps: usize) -> u64 {
        let mut counts = BTreeMap::<u8, u64>::new();
        let mut bytes = self.polymer.iter().copied();
        let mut left = bytes.next().unwrap();
        counts.insert(left, 1);
        let mut cache = Cache::new();
        for right in bytes {
            *counts.entry(right).or_insert(0) += 1;
            self.counts_after_expanding(num_steps, left, right, &mut counts, &mut cache);
            left = right;
        }
        let min_count = counts.values().min().unwrap();
        let max_count = counts.values().max().unwrap();
        max_count - min_count
    }

    fn counts_after_expanding(
        &self,
        steps: usize,
        left: u8,
        right: u8,
        result_counts: &mut BTreeMap<u8, u64>,
        cache: &mut Cache,
    ) {
        if steps == 0 {
            return;
        }
        if let Some(counts) = cache.counts.get(&(steps, left, right)) {
            combine_counts(result_counts, counts);
            return;
        };
        let middle = match self.rules.get(&(left, right)) {
            None => return,
            Some(&middle) => middle,
        };

        let mut counts = BTreeMap::new();
        counts.insert(middle, 1);
        self.counts_after_expanding(steps - 1, left, middle, &mut counts, cache);
        self.counts_after_expanding(steps - 1, middle, right, &mut counts, cache);
        combine_counts(result_counts, &counts);
        cache.counts.insert((steps, left, right), counts);
    }
}

struct Cache {
    counts: HashMap<(usize, u8, u8), BTreeMap<u8, u64>>,
}
impl Cache {
    fn new() -> Cache {
        Cache {
            counts: HashMap::new(),
        }
    }
}

fn combine_counts(l: &mut BTreeMap<u8, u64>, r: &BTreeMap<u8, u64>) {
    for (&key, val) in r {
        *l.entry(key).or_insert(0) += val;
    }
}

fn part_1(input: &str) -> u64 {
    let problem = input.parse::<Problem>().unwrap();
    problem.counts_after(10)
}

#[test]
fn test_part_1() {
    let input = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
    "
    .trim();
    assert_eq!(part_1(input), 1588);
    assert_eq!(part_1(include_str!("day14.txt")), 5656);
}

fn part_2(input: &str) -> u64 {
    let problem = input.parse::<Problem>().unwrap();
    problem.counts_after(40)
}

#[test]
fn test_part_2() {
    let input = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
    "
    .trim();
    assert_eq!(part_2(input), 2_188_189_693_529);
    assert_eq!(part_2(include_str!("day14.txt")), 12_271_437_788_530);
}
