fn parse_line(line: &str) -> Vec<i64> {
    line.split(' ').map(|v| v.parse().unwrap()).collect()
}

fn extrapolate_last(cur: &[i64]) -> i64 {
    if cur.iter().all(|&v| v == 0) {
        return 0;
    }

    let next: Vec<_> = cur.windows(2).map(|v| v[1] - v[0]).collect();

    let next_v = extrapolate_last(&next);
    cur.last().unwrap() + next_v
}

fn extrapolate_first(cur: &[i64]) -> i64 {
    if cur.iter().all(|&v| v == 0) {
        return 0;
    }

    let next: Vec<_> = cur.windows(2).map(|v| v[1] - v[0]).collect();

    let next_v = extrapolate_first(&next);
    cur.first().unwrap() - next_v
}

fn part1(data: &str) -> i64 {
    data.trim()
        .lines()
        .map(parse_line)
        .map(|v| extrapolate_last(&v))
        .sum()
}

fn part2(data: &str) -> i64 {
    data.trim()
        .lines()
        .map(parse_line)
        .map(|v| extrapolate_first(&v))
        .sum()
}

fn main() {
    println!("Day 9");
    let data = std::fs::read_to_string("input/day09.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#;

    #[test]
    fn day09_part1() {
        assert_eq!(part1(DATA), 114);
    }

    #[test]
    fn day09_part2() {
        assert_eq!(part2(DATA), 2);
    }
}
