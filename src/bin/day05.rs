use std::{collections::HashMap, ops::Range};

struct Almanac<'a> {
    seeds: Vec<usize>,
    maps: HashMap<&'a str, Map<'a>>,
}

impl<'a> Almanac<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut it = input.trim().lines();
        let seeds = it
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();
        assert_eq!(it.next(), Some(""));

        let mut maps = HashMap::new();

        'parse_map: loop {
            let map = Map::from_input(it.next().unwrap());
            let map = match maps.entry(map.from) {
                std::collections::hash_map::Entry::Vacant(v) => v.insert(map),
                _ => unreachable!(),
            };

            for line in it.by_ref() {
                if line.is_empty() {
                    continue 'parse_map;
                }
                map.entries.push(MapEntry::from_input(line));
            }
            break;
        }

        maps.values_mut()
            .for_each(|v| v.entries.sort_by_key(|e| e.src_start));

        Self { seeds, maps }
    }
}

struct Map<'a> {
    from: &'a str,
    to: &'a str,
    entries: Vec<MapEntry>,
}

impl<'a> Map<'a> {
    fn from_input(input: &'a str) -> Self {
        let (from, remain) = input.split_once("-to-").unwrap();
        let (to, remain) = remain.split_once(' ').unwrap();
        assert_eq!(remain, "map:");
        Self {
            from,
            to,
            entries: Vec::new(),
        }
    }

    fn map_entry(&self, value: usize) -> Range<usize> {
        match self.entries.binary_search_by_key(&value, |v| v.src_start) {
            Ok(i) => {
                let entry = &self.entries[i];
                entry.dst_start..entry.dst_start + entry.len
            }
            Err(i) => {
                if i == 0 {
                    value..self.entries[0].src_start
                } else {
                    let entry = &self.entries[i - 1];
                    let offset = value - entry.src_start;
                    if offset < entry.len {
                        entry.dst_start + offset..entry.dst_start + entry.len
                    } else if let Some(next) = self.entries.get(i) {
                        value..next.src_start
                    } else {
                        value..usize::MAX
                    }
                }
            }
        }
    }
}

struct MapEntry {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

impl MapEntry {
    fn from_input(input: &str) -> Self {
        let mut it = input.split(' ').map(|v| v.parse().unwrap());
        let ret = Self {
            dst_start: it.next().unwrap(),
            src_start: it.next().unwrap(),
            len: it.next().unwrap(),
        };
        assert_eq!(it.next(), None);
        ret
    }
}

fn part1(data: &str) -> usize {
    let almanac = Almanac::from_input(data);
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            let mut map = &almanac.maps["seed"];
            let mut cur = seed;
            loop {
                cur = map.map_entry(cur).start;
                if map.to == "location" {
                    return cur;
                }
                map = &almanac.maps[map.to];
            }
        })
        .min()
        .unwrap()
}

fn part2(data: &str) -> usize {
    let almanac = Almanac::from_input(data);
    almanac
        .seeds
        .chunks(2)
        .map(|range| {
            let mut cur_seed = range[0];
            let mut remain = range[1];
            let mut cur_result = usize::MAX;
            while remain > 0 {
                let mut count = remain;
                let mut map = &almanac.maps["seed"];
                let mut cur = cur_seed;
                loop {
                    let range = map.map_entry(cur);
                    count = count.min(range.len());
                    cur = range.start;
                    if map.to == "location" {
                        cur_result = cur_result.min(cur);
                        break;
                    }
                    map = &almanac.maps[map.to];
                }

                cur_seed += count;
                remain -= count;
            }
            cur_result
        })
        .min()
        .unwrap()
}

fn main() {
    println!("Day 2");
    let data = std::fs::read_to_string("input/day05.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn day05_part1() {
        assert_eq!(part1(PART1), 35);
    }

    #[test]
    fn day05_part2() {
        assert_eq!(part2(PART1), 46);
    }
}
