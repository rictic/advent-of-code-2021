use std::collections::VecDeque;

use anyhow::{Context, Result};
use num_bigint::BigUint;

fn count_lanternfish(input: &str, num_days: u64) -> Result<BigUint> {
    let init: [BigUint; 9] = Default::default();
    let mut num_fish_each_day_until_spawn: VecDeque<BigUint> = VecDeque::from(init);
    for days in input.split(',') {
        let days = days.parse::<usize>().context("parsing input number")?;
        if days > num_fish_each_day_until_spawn.len() {
            anyhow::bail!(
                "input number is too large! Expected at most {} but got {}",
                num_fish_each_day_until_spawn.len(),
                days
            );
        }
        num_fish_each_day_until_spawn[days] += Into::<BigUint>::into(1 as u64);
    }
    for _ in 0..num_days {
        let num_spawning = num_fish_each_day_until_spawn.pop_front().unwrap();
        num_fish_each_day_until_spawn[6] += &num_spawning;
        num_fish_each_day_until_spawn.push_back(num_spawning);
    }

    Ok(num_fish_each_day_until_spawn.into_iter().sum())
}

#[test]
fn test_part_1() {
    assert_eq!(count_lanternfish("3,4,3,1,2", 1).unwrap(), 5u64.into());
    assert_eq!(count_lanternfish("3,4,3,1,2", 2).unwrap(), 6u64.into());
    assert_eq!(count_lanternfish("3,4,3,1,2", 18).unwrap(), 26u64.into());
    assert_eq!(count_lanternfish("3,4,3,1,2", 80).unwrap(), 5_934u64.into());
    assert_eq!(
        count_lanternfish(include_str!("./day6.txt"), 80).unwrap(),
        380_243u64.into()
    );
    let big = count_lanternfish("3,4,3,1,2", 9999999).unwrap().to_string();
    assert_eq!(big.len(), 378346);
    assert!(big.starts_with("4182599183"));
    assert!(big.ends_with("6707352532"));
}

#[test]
fn test_part_2() {
    assert_eq!(
        count_lanternfish("3,4,3,1,2", 256).unwrap(),
        26_984_457_539u64.into()
    );
    assert_eq!(
        count_lanternfish(include_str!("./day6.txt"), 256).unwrap(),
        1_708_791_884_591u64.into()
    );
}
