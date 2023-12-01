fn part1(data: &str) -> u64 {
    data.trim()
        .lines()
        .map(|l| {
            let it = l
                .as_bytes()
                .iter()
                .copied()
                .filter(|b| b.is_ascii_digit())
                .map(|b| u64::from(b - b'0'));
            (it.clone().next().unwrap(), it.last().unwrap())
        })
        .map(|(a, b)| a * 10 + b)
        .sum()
}

fn part2(data: &str) -> usize {
    static NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    data.trim()
        .lines()
        .map(|l| {
            let it = l
                .as_bytes()
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(i, b)| {
                    if b.is_ascii_digit() {
                        Some(usize::from(b - b'0'))
                    } else if let Some(x) = NUMBERS
                        .iter()
                        .position(|num| l.as_bytes()[i..].starts_with(num.as_bytes()))
                    {
                        Some(x + 1)
                    } else {
                        None
                    }
                });
            (it.clone().next().unwrap(), it.last().unwrap())
        })
        .map(|(a, b)| a * 10 + b)
        .sum()
}

fn main() {
    println!("Day 1");
    let data = std::fs::read_to_string("input/day1.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const PART2: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn example() {
        assert_eq!(part1(PART1), 142);
        assert_eq!(part2(PART2), 281);
    }
}
