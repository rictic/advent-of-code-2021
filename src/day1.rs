fn parse(input: &str) -> std::io::Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<_, _>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn num_increases(input: &str) -> std::io::Result<usize> {
    let mut num_increases = 0;
    let mut prev: Option<i64> = None;
    for num in parse(input)? {
        if let Some(prev) = prev {
            if num > prev {
                num_increases += 1;
            }
        }
        prev = Some(num);
    }
    Ok(num_increases)
}

fn unordered_windows<Iter, T, const L: usize>(it: Iter) -> impl Iterator<Item = [T; L]>
where
    Iter: Iterator<Item = T>,
    T: Copy + std::fmt::Debug + Default,
{
    let buffer = [T::default(); L];
    it.scan(
        (buffer, 0 as usize, false),
        move |(buffer, idx, filled), item| {
            buffer[*idx] = item;
            *idx = *idx + 1;
            if *idx == L {
                *filled = true;
                *idx = 0;
            }
            if *filled {
                Some(Some(buffer.clone()))
            } else {
                Some(None)
            }
        },
    )
    .filter_map(|x| x)
}

fn num_window_increases(input: &str) -> std::io::Result<usize> {
    let mut num_increases = 0;
    let mut prev: Option<i64> = None;
    let nums = parse(input)?;
    for window in unordered_windows::<_, _, 3>(nums.iter().copied()) {
        let sum = window.iter().sum();
        if let Some(prev) = prev {
            if sum > prev {
                num_increases += 1;
            }
        }
        prev = Some(sum);
    }
    Ok(num_increases)
}

#[test]
fn test_num_decreases() -> std::io::Result<()> {
    let example = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(num_increases(example)?, 7);
    assert_eq!(num_increases(include_str!("day1.txt"))?, 1766);

    Ok(())
}

#[test]
fn test_windowed_increases() -> std::io::Result<()> {
    let example = "199
200
208
210
200
207
240
269
260
263";
    assert_eq!(num_window_increases(example)?, 5);
    assert_eq!(num_window_increases(include_str!("day1.txt"))?, 1797);
    Ok(())
}
