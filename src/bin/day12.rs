use std::collections::HashMap;

struct State<'a, 'cache> {
    springs: &'a [u8],
    counts: &'a [usize],
    cache: &'cache mut HashMap<(usize, usize), usize>,
}

impl<'a, 'cache> State<'a, 'cache> {
    fn count_arrangements(&mut self, initial_skip: usize) -> usize {
        let max_to_skip = self
            .springs
            .iter()
            .position(|&b| b == b'#')
            .unwrap_or(self.springs.len());

        let to_skip = if initial_skip <= max_to_skip {
            self.springs[initial_skip..max_to_skip]
                .iter()
                .position(|&b| b != b'.')
                .map(|v| initial_skip + v)
                .unwrap_or(max_to_skip)
        } else if self.counts.is_empty() && self.springs.len() == max_to_skip {
            return 1;
        } else {
            return 0;
        };

        if self.counts.is_empty() {
            // If there are no more counts, and the remainder is not '#', we have a match
            if max_to_skip == self.springs.len() {
                return 1;
            } else {
                return 0;
            }
        } else if self.springs.is_empty() {
            return 0;
        }

        let springs = self.springs.get(to_skip..).unwrap_or(b"");

        let (&count, remaining_counts) = self.counts.split_first().unwrap();
        let mut ret = 0;
        let sequence = springs.split(|&b| b == b'.').next().unwrap_or(b"");

        if count <= sequence.len() {
            for i in count..=sequence.len() {
                let (cur, remainder) = springs.split_at(i);
                let res = if let Some(&res) =
                    self.cache.get(&(remainder.len(), remaining_counts.len()))
                {
                    res
                } else {
                    let res = State {
                        springs: remainder,
                        counts: remaining_counts,
                        cache: self.cache,
                    }
                    .count_arrangements(1);
                    self.cache
                        .insert((remainder.len(), remaining_counts.len()), res);
                    res
                };

                ret += res;

                // If the current sequence starts with a damaged spring, we only need to check the first
                if cur[i - count] == b'#' {
                    break;
                }
            }
        }

        if !sequence.contains(&b'#') {
            let remain = &springs[sequence.len()..];
            ret += State {
                springs: remain,
                counts: self.counts,
                cache: self.cache,
            }
            .count_arrangements(0);
        }
        ret
    }
}

fn part1(data: &str) -> usize {
    data.trim()
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let counts: Vec<_> = counts.split(',').map(|v| v.parse().unwrap()).collect();
            State {
                springs: springs.as_bytes(),
                counts: &counts,
                cache: &mut HashMap::new(),
            }
            .count_arrangements(0)
        })
        .sum()
}

fn part2(data: &str) -> usize {
    data.trim()
        .lines()
        .map(|line| {
            let (raw_springs, counts) = line.split_once(' ').unwrap();
            let mut springs = Vec::with_capacity((raw_springs.len() + 1) * 5);
            for _ in 0..5 {
                springs.extend_from_slice(raw_springs.as_bytes());
                springs.push(b'?');
            }
            springs.pop();
            let counts: Vec<usize> = counts
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            State {
                springs: &springs,
                counts: &counts,
                cache: &mut HashMap::new(),
            }
            .count_arrangements(0)
        })
        .sum()
}

fn main() {
    println!("Day 12");
    let data = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn day12_part1() {
        assert_eq!(part1(DATA1), 21);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(DATA1), 525152);
    }
}
