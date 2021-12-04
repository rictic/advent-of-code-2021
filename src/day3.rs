use anyhow::{anyhow, Result};

fn extract_counts(input: &Vec<String>) -> Result<(Vec<usize>, usize)> {
    let mut counts = vec![];
    let mut line_count = 0;
    for line in input {
        for (idx, c) in line.chars().enumerate() {
            let i = c.to_digit(2);
            // convert option to result
            let i = i.ok_or_else(|| anyhow::anyhow!("invalid bit char: {}", c))?;
            if idx >= counts.len() {
                counts.push(0);
            }
            if i == 1 {
                counts[idx] += 1;
            }
        }
        line_count += 1;
    }
    Ok((counts, line_count))
}

fn extract_gamma_and_epsilon(input: &str) -> Result<(u64, u64)> {
    let lines = input.lines().map(|l| String::from(l)).collect();
    let (counts, line_count) = extract_counts(&lines)?;
    // for each bit, if the count at that bit is > half of the number of lines, then it's a 1
    let mut gamma_rate: u64 = 0;
    let mut epsilon_rate = 0;
    for count in counts.iter() {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if count > &(line_count / 2) {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }
    Ok((gamma_rate, epsilon_rate))
}

fn part_1(input: &str) -> Result<u64> {
    let (gamma_rate, epsilon_rate) = extract_gamma_and_epsilon(input)?;
    let power_consumption = gamma_rate * epsilon_rate;
    Ok(power_consumption)
}

#[test]
fn test_part1() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(extract_gamma_and_epsilon(input).unwrap(), (22, 9));
    assert_eq!(part_1(input).unwrap(), 198);

    assert_eq!(part_1(include_str!("./day3.txt")).unwrap(), 693_486);
}

fn extract_oxygen_and_co2(input: &str) -> Result<(u64, u64)> {
    let mut o2_candidates: Vec<String> = input.lines().map(|l| String::from(l)).collect();
    let mut co2_candidates = o2_candidates.clone();
    let mut pos: usize = 0;
    while o2_candidates.len() > 1 {
        let (counts, line_count) = extract_counts(&o2_candidates)?;
        let majority_ones = match counts.get(pos) {
            Some(&count) => count >= ((line_count as f64) / 2.0).ceil() as usize,
            None => {
                return Err(anyhow!(
                    "Ran out of diagnostics by the time we looked at bit {} of the oxygen line",
                    pos
                ))
            }
        };
        o2_candidates = o2_candidates
            .into_iter()
            .filter(|line| {
                let char = line.chars().nth(pos);
                if majority_ones {
                    char == Some('1')
                } else {
                    char == Some('0')
                }
            })
            .collect();
        pos += 1;
    }
    let mut pos: usize = 0;
    while co2_candidates.len() > 1 {
        let (counts, line_count) = extract_counts(&co2_candidates)?;
        let majority_zeros = match counts.get(pos) {
            Some(&count) => count >= ((line_count as f64) / 2.0).ceil() as usize,
            None => {
                return Err(anyhow!(
                    "Ran out of diagnostics by the time we looked at bit {} of co2 scrubbers",
                    pos
                ))
            }
        };
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|line| {
                let char = line.chars().nth(pos);
                if majority_zeros {
                    char == Some('0')
                } else {
                    char == Some('1')
                }
            })
            .collect();
        pos += 1;
    }
    let oxygen_rate = match o2_candidates.get(0) {
        Some(line) => {
            // parse line as a binary string into a number
            u64::from_str_radix(line, 2).map_err(|_| anyhow!("invalid binary number: {}", line))?
        }
        None => {
            return Err(anyhow!(
                "Ran out of diagnostics by the time we looked at bit {} of the oxygen line",
                pos
            ))
        }
    };
    let co2_rate = match co2_candidates.get(0) {
        Some(line) => {
            // parse line as a binary string into a number
            u64::from_str_radix(line, 2).map_err(|_| anyhow!("invalid binary number: {}", line))?
        }
        None => {
            return Err(anyhow!(
                "Ran out of diagnostics by the time we looked at bit {} of c02 scrubbers",
                pos
            ))
        }
    };

    // for each bit, if the count at that bit is > half of the number of lines, then it's a 1
    Ok((oxygen_rate, co2_rate))
}

fn part_2(input: &str) -> Result<u64> {
    let (oxygen_rate, co2_rate) = extract_oxygen_and_co2(input)?;
    let power_consumption = oxygen_rate * co2_rate;
    Ok(power_consumption)
}

#[test]
fn test_part2() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(extract_oxygen_and_co2(input).unwrap(), (23, 10));
    assert_eq!(part_2(input).unwrap(), 230);

    assert_eq!(part_2(include_str!("./day3.txt")).unwrap(), 3379326);
}
