use anyhow::{Context, Result};

fn part_1(input: &str, num_days: usize) -> Result<u64> {
    let mut num_fish_each_day_until_spawn = [0 as u64; 9];
    for days in input.split(',') {
        let days = days.parse::<usize>().context("parsing input number")?;
        if days > num_fish_each_day_until_spawn.len() {
            anyhow::bail!(
                "input number is too large! Expected at most {} but got {}",
                num_fish_each_day_until_spawn.len(),
                days
            );
        }
        num_fish_each_day_until_spawn[days] += 1;
    }
    for _ in 0..num_days {
        let num_spawning = num_fish_each_day_until_spawn[0];
        for i in 1..num_fish_each_day_until_spawn.len() {
            num_fish_each_day_until_spawn[i - 1] = num_fish_each_day_until_spawn[i];
        }
        num_fish_each_day_until_spawn[8] = num_spawning;
        num_fish_each_day_until_spawn[6] += num_spawning;
    }

    Ok(num_fish_each_day_until_spawn.into_iter().sum())
}

#[test]
fn test_part_1() {
    assert_eq!(part_1("3,4,3,1,2", 1).unwrap(), 5);
    assert_eq!(part_1("3,4,3,1,2", 2).unwrap(), 6);
    assert_eq!(part_1("3,4,3,1,2", 18).unwrap(), 26);
    assert_eq!(part_1("3,4,3,1,2", 80).unwrap(), 5_934);
    assert_eq!(part_1(include_str!("./day6.txt"), 80).unwrap(), 380_243);
}

#[test]
fn test_part_2() {
    assert_eq!(part_1("3,4,3,1,2", 256).unwrap(), 26_984_457_539);
    assert_eq!(
        part_1(include_str!("./day6.txt"), 256).unwrap(),
        1_708_791_884_591
    );
}
