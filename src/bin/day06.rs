struct Race {
    time: usize,
    dist: usize,
}

fn parse_line<'a>(line: &'a str, prefix: &str) -> impl Iterator<Item = usize> + 'a {
    line.strip_prefix(prefix)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
}

fn parse_line2<'a>(line: &'a str, prefix: &str) -> usize {
    line.strip_prefix(prefix)
        .unwrap()
        .trim()
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn parse_races(data: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = data.trim().lines();
    let time = parse_line(lines.next().unwrap(), "Time:");
    let dist = parse_line(lines.next().unwrap(), "Distance:");
    assert_eq!(lines.next(), None);

    time.zip(dist).map(|(time, dist)| Race { time, dist })
}

fn part1(data: &str) -> usize {
    parse_races(data)
        .map(|race| {
            let mut count = 0;
            for hold_ms in 1..race.time - 1 {
                let dist = hold_ms * (race.time - hold_ms);
                if dist > race.dist {
                    count += 1;
                }
            }
            count
        })
        .product()
}

fn part2(data: &str) -> usize {
    let mut lines = data.trim().lines();
    let time = parse_line2(lines.next().unwrap(), "Time:");
    let record = parse_line2(lines.next().unwrap(), "Distance:");

    let mut count = 0;
    for hold_ms in 1..time - 1 {
        let dist = hold_ms * (time - hold_ms);
        if dist > record {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("Day 2");
    let data = std::fs::read_to_string("input/day06.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn day06_part1() {
        assert_eq!(part1(PART1), 288);
    }

    #[test]
    fn day06_part2() {
        assert_eq!(part2(PART1), 71503);
    }
}
