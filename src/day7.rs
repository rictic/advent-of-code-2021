use anyhow::{anyhow, Context, Result};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn part_1(input: &str) -> Result<i64> {
    let vals = input
        .split(",")
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| anyhow!("Failed to parse {}", s))
        })
        .collect::<Result<Vec<i64>>>()?;
    let min = *vals.iter().min().ok_or(anyhow!("Empty input"))?;
    let max = *vals.iter().max().ok_or(anyhow!("Empty input"))?;
    (min..=max)
        .into_iter()
        .map(|target| vals.iter().map(|&val| (val - target).abs()).sum::<i64>())
        .min()
        .ok_or(anyhow!("Empty input"))
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("16,1,2,0,4,2,7,1,2,14").unwrap(), 37);
    assert_eq!(part_1(include_str!("./day7.txt")).unwrap(), 335_271);
}

fn part_2(input: &str) -> Result<i64> {
    let vals = input
        .split(",")
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| anyhow!("Failed to parse {}", s))
        })
        .collect::<Result<Vec<i64>>>()?;
    let min = *vals.iter().min().ok_or(anyhow!("Empty input"))?;
    let max = *vals.iter().max().ok_or(anyhow!("Empty input"))?;
    (min..=max)
        .into_par_iter()
        .map(|target| {
            vals.par_iter()
                .map(|&val| {
                    let distance = (val - target).abs();
                    (0..=distance).sum::<i64>()
                })
                .sum::<i64>()
        })
        .min()
        .ok_or(anyhow!("Empty input"))
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("16,1,2,0,4,2,7,1,2,14").unwrap(), 168);
    assert_eq!(part_2(include_str!("./day7.txt")).unwrap(), 95_851_339);
}
