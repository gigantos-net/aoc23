use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

fn roll_row_of_tiles<'a, 'b>(
    it: impl ExactSizeIterator<Item = (&'a mut Tile, &'b mut Tile)>,
) -> bool {
    let mut changes_made = false;
    for (prev, cur) in it {
        match cur {
            Tile::Empty | Tile::Fixed => (),
            Tile::Roll => match prev {
                Tile::Roll | Tile::Fixed => (),
                Tile::Empty => {
                    changes_made = true;
                    *prev = std::mem::replace(cur, Tile::Empty)
                }
            },
        }
    }
    changes_made
}

#[derive(Debug, Hash)]
enum Tile {
    Empty,
    Roll,
    Fixed,
}

struct Map {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_input(data: &str) -> Self {
        let data = data.trim();
        let width = data.lines().next().unwrap().len();
        let data: Vec<_> = data
            .trim()
            .lines()
            .map(|l| l.as_bytes())
            .flatten()
            .copied()
            .map(|v| match v {
                b'.' => Tile::Empty,
                b'O' => Tile::Roll,
                b'#' => Tile::Fixed,
                _ => unreachable!(),
            })
            .collect();
        let height = data.len() / width;
        Self {
            data,
            width,
            height,
        }
    }

    fn tilt_north(&mut self) {
        for y in 0..self.height {
            let i = self.width + (y * self.width);
            let mut it = self
                .data
                .get_mut(..i)
                .unwrap_or_default()
                .rchunks_mut(self.width)
                .peekable();

            while let Some(cur_row) = it.next() {
                if let Some(prev_row) = it.peek_mut() {
                    if !roll_row_of_tiles(prev_row.iter_mut().zip(cur_row.iter_mut())) {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.height).rev() {
            let i = y * self.width;
            let mut it = self
                .data
                .get_mut(i..)
                .unwrap_or_default()
                .chunks_mut(self.width)
                .peekable();

            while let Some(cur_row) = it.next() {
                if let Some(prev_row) = it.peek_mut() {
                    if !roll_row_of_tiles(prev_row.iter_mut().zip(cur_row.iter_mut())) {
                        break;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for max_x in 0..self.width - 1 {
            for x in (0..=max_x).rev() {
                let it = self
                    .data
                    .get_mut(x..)
                    .unwrap_or_default()
                    .chunks_mut(self.width)
                    .map(|v| {
                        let (prev, rem) = v.split_first_mut().unwrap();
                        let cur = rem.first_mut().unwrap();
                        (prev, cur)
                    });

                if !roll_row_of_tiles(it) {
                    break;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for min_x in (0..self.width - 1).rev() {
            for x in min_x..self.width - 1 {
                let it = self
                    .data
                    .get_mut(x..)
                    .unwrap_or_default()
                    .chunks_mut(self.width)
                    .map(|v| {
                        let (cur, rem) = v.split_first_mut().unwrap();
                        let prev = rem.first_mut().unwrap();
                        (prev, cur)
                    });

                if !roll_row_of_tiles(it) {
                    break;
                }
            }
        }
    }

    fn calc_north_load(&self) -> usize {
        self.data
            .chunks(self.width)
            .enumerate()
            .map(|(i, row)| {
                row.iter().filter(|v| matches!(v, Tile::Roll)).count() * (self.height - i)
            })
            .sum()
    }

    fn cur_hash(&self) -> u64 {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        Tile::hash_slice(&self.data, &mut h);
        h.finish()
    }
}

fn part1(data: &str) -> usize {
    let mut map = Map::from_input(data);
    map.tilt_north();
    map.calc_north_load()
}

fn part2(data: &str) -> usize {
    const ITERATIONS: usize = 1_000_000_000;
    let mut map = Map::from_input(data);
    let mut seen_hashes = HashMap::new();
    for i in 0..ITERATIONS {
        map.tilt_north();
        map.tilt_west();
        map.tilt_south();
        map.tilt_east();
        if let Some(prev_i) = seen_hashes.insert(map.cur_hash(), i) {
            let iterations_per = i - prev_i;
            let remaining = ITERATIONS - i;
            if (remaining % iterations_per) == 1 {
                break;
            }
        }
    }
    map.calc_north_load()
}

fn main() {
    println!("Day 14");
    let data = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn day14_part1() {
        assert_eq!(part1(DATA1), 136);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(part2(DATA1), 64);
    }
}
