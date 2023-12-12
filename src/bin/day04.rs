use std::collections::HashSet;

struct Set {
    num: HashSet<usize>,
}

impl Set {
    fn from_input(input: &str) -> Self {
        Self {
            num: input
                .as_bytes()
                .chunks(3)
                .map(|v| std::str::from_utf8(v).unwrap())
                .map(|v| v.trim().parse().unwrap())
                .collect(),
        }
    }
}

struct Card {
    win: Set,
    your: Set,
}

impl Card {
    fn from_line(line: &str) -> Option<Self> {
        let (win, your) = line.split_once(": ")?.1.split_once(" | ")?;
        Some(Self {
            win: Set::from_input(win),
            your: Set::from_input(your),
        })
    }

    fn num_winners(&self) -> usize {
        self.win.num.intersection(&self.your.num).count()
    }
}

fn part1(data: &str) -> usize {
    data.trim()
        .lines()
        .map(|line| Card::from_line(line).unwrap())
        .map(|card| {
            let count = card.num_winners();
            if count > 0 {
                1 << count - 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &str) -> usize {
    struct Entry {
        value: usize,
        count: usize,
    }

    let mut sums: Vec<Entry> = data
        .trim()
        .lines()
        .map(|line| Entry {
            value: Card::from_line(line).unwrap().num_winners(),
            count: 1,
        })
        .collect();

    for i in 0..sums.len() {
        let (cur, remain) = sums[i..].split_first_mut().unwrap();
        for x in &mut remain[..cur.value] {
            x.count += cur.count;
        }
    }

    sums.iter().map(|v| v.count).sum()
}

fn main() {
    println!("Day 2");
    let data = std::fs::read_to_string("input/day04.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn day04_part1() {
        assert_eq!(part1(PART1), 13);
    }

    #[test]
    fn day04_part2() {
        assert_eq!(part2(PART1), 30);
    }
}
