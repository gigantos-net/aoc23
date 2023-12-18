struct Map<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
    stride: usize,
}

impl<'a> Map<'a> {
    fn from_input(data: &'a str) -> Self {
        let width = data.lines().next().unwrap().len();
        let stride = width + 1;
        Self {
            data: data.as_bytes(),
            width,
            height: data.len() / stride,
            stride,
        }
    }

    fn mirror_iter_h(
        &self,
        y: usize,
    ) -> impl Iterator<Item = (impl Iterator<Item = u8> + '_, impl Iterator<Item = u8> + '_)> + '_
    {
        let (a, b) = self.data.split_at(y * self.stride);
        a.rchunks(self.stride)
            .map(|v| v[..self.width].iter().copied())
            .zip(
                b.chunks(self.stride)
                    .map(|v| v[..self.width].iter().copied()),
            )
    }

    fn mirror_iter_v(
        &self,
        x: usize,
    ) -> impl Iterator<Item = (impl Iterator<Item = u8> + '_, impl Iterator<Item = u8> + '_)> + '_
    {
        (0..x).rev().zip(x..self.width).map(move |(x1, x2)| {
            (
                self.data[x1..].chunks(self.stride).map(|v| v[0]),
                self.data[x2..].chunks(self.stride).map(|v| v[0]),
            )
        })
    }
}

fn part1(data: &str) -> usize {
    data.trim()
        .split("\n\n")
        .map(Map::from_input)
        .map(|map| {
            if let Some(y) = (1..=map.height).find(|&y| map.mirror_iter_h(y).all(|(a, b)| a.eq(b)))
            {
                y * 100
            } else if let Some(x) =
                (1..=map.width).find(|&x| map.mirror_iter_v(x).all(|(a, b)| a.eq(b)))
            {
                x
            } else {
                unreachable!()
            }
        })
        .sum()
}

fn part2(data: &str) -> usize {
    #[derive(Default)]
    struct CmpState {
        num_mismatch: usize,
    }

    impl CmpState {
        fn compare(&mut self, a: impl Iterator<Item = u8>, b: impl Iterator<Item = u8>) -> bool {
            if self.num_mismatch > 1 {
                return false;
            }

            for (a, b) in a.zip(b) {
                if a != b {
                    self.num_mismatch += 1;
                    if self.num_mismatch > 1 {
                        return false;
                    }
                }
            }

            true
        }
    }

    data.trim()
        .split("\n\n")
        .map(Map::from_input)
        .map(|map| {
            if let Some(y) = (1..=map.height).find(|&y| {
                let mut state = CmpState::default();
                map.mirror_iter_h(y).all(|(a, b)| state.compare(a, b));
                state.num_mismatch == 1
            }) {
                y * 100
            } else if let Some(x) = (1..=map.width).find(|&x| {
                let mut state = CmpState::default();
                map.mirror_iter_v(x).all(|(a, b)| state.compare(a, b));
                state.num_mismatch == 1
            }) {
                x
            } else {
                unreachable!()
            }
        })
        .sum()
}

fn main() {
    println!("Day 13");
    let data = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn day13_part1() {
        assert_eq!(part1(DATA1), 405);
    }

    #[test]
    fn day13_part2() {
        assert_eq!(part2(DATA1), 400);
    }
}
