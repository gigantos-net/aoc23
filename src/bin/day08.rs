use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Id<'a>(&'a str);

impl<'a> Id<'a> {
    fn from_input(input: &'a str) -> Self {
        assert_eq!(input.len(), 3);
        for b in input.as_bytes() {
            assert!(matches!(b, b'0'..=b'9' | b'A'..=b'Z'), "{input:?}");
        }
        Self(input)
    }
}

#[derive(Default)]
struct Map<'a> {
    entries: HashMap<Id<'a>, (Id<'a>, Id<'a>)>,
}

impl<'a> Map<'a> {
    fn add(&mut self, line: &'a str) {
        let (entry, remain) = line.split_once(" = ").unwrap();
        let (l, r) = remain
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        self.entries.insert(
            Id::from_input(entry),
            (Id::from_input(l), Id::from_input(r)),
        );
    }

    fn count_steps(&self, instructions: &str, start: Id, is_end: impl Fn(Id) -> bool) -> usize {
        let mut cur = start;
        let mut steps = 0;
        for inst in std::iter::repeat(instructions.as_bytes()).flatten() {
            steps += 1;
            match inst {
                b'L' => cur = self.entries[&cur].0,
                b'R' => cur = self.entries[&cur].1,
                _ => unreachable!(),
            }
            if is_end(cur) {
                break;
            }
        }
        steps
    }
}

fn parse_input(input: &str) -> (&str, Map) {
    let mut it = input.trim().lines();
    let instructions = it.next().unwrap();
    assert_eq!(it.next(), Some(""));
    let mut map = Map::default();

    for row in it {
        map.add(row);
    }

    (instructions, map)
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let rem = a % b;
        a = std::mem::replace(&mut b, rem);
    }
    a
}

fn least_common_multiple(list: &[usize]) -> usize {
    let mut a = list[0];
    for &b in &list[1..] {
        a = (a * b) / greatest_common_divisor(a, b);
    }
    a
}

fn part1(data: &str) -> usize {
    let (instructions, map) = parse_input(data);
    let end = Id::from_input("ZZZ");
    map.count_steps(instructions, Id::from_input("AAA"), |v| v == end)
}

fn part2(data: &str) -> usize {
    let (instructions, map) = parse_input(data);
    let mut counts = Vec::new();
    for node in map.entries.keys().copied() {
        if node.0.ends_with('A') {
            counts.push(map.count_steps(instructions, node, |v| v.0.ends_with('Z')));
        }
    }

    least_common_multiple(&counts)
}

fn main() {
    println!("Day 8");
    let data = std::fs::read_to_string("input/day08.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const DATA2: &str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const DATA3: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn day08_part1() {
        assert_eq!(part1(DATA), 2);
        assert_eq!(part1(DATA2), 6);
    }

    #[test]
    fn day08_part2() {
        assert_eq!(part2(DATA3), 6);
    }
}
