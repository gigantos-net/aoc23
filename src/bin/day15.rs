use std::collections::HashMap;

fn hash_str(v: &str) -> usize {
    v.as_bytes()
        .iter()
        .copied()
        .fold(0u8, |state, v| {
            state.overflowing_add(v).0.overflowing_mul(17).0
        })
        .into()
}
fn part1(data: &str) -> usize {
    data.trim().split(',').map(hash_str).sum()
}

fn part2(data: &str) -> usize {
    let mut maps = [(); 256].map(|()| HashMap::new());
    let mut index = 0;
    for cur in data.trim().split(',') {
        match cur.split_once(['-', '=']) {
            Some((lens, "")) => {
                maps[hash_str(lens)].remove(lens);
            }
            Some((lens, focal)) => {
                maps[hash_str(lens)]
                    .entry(lens)
                    .or_insert_with(|| {
                        index += 1;
                        (index, 0)
                    })
                    .1 = focal.parse().unwrap()
            }
            None => unreachable!(),
        }
    }
    maps.iter()
        .enumerate()
        .filter(|(_, map)| !map.is_empty())
        .flat_map(|(box_index, map)| {
            let box_nr = box_index + 1;
            let mut sorted: Vec<_> = map.values().copied().collect();
            sorted.sort();
            sorted
                .into_iter()
                .enumerate()
                .map(move |(slot, (_, focal))| box_nr * (slot + 1) * focal)
        })
        .sum()
}

fn main() {
    println!("Day 15");
    let data = std::fs::read_to_string("input/day15.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;

    #[test]
    fn day15_part1_hash() {
        assert_eq!(hash_str("HASH"), 52);
        assert_eq!(hash_str("rn=1"), 30);
        assert_eq!(hash_str("cm-"), 253);
    }

    #[test]
    fn day15_part1() {
        assert_eq!(part1(DATA1), 1320);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(part2(DATA1), 145);
    }
}
