enum Tile {
    Galaxy,
    Empty { h: usize, v: usize },
}

struct Universe {
    grid: Vec<Tile>,
    width: usize,
}

impl Universe {
    fn from_input(input: &str) -> Self {
        Self {
            grid: input
                .trim()
                .as_bytes()
                .iter()
                .filter_map(|b| match b {
                    b'.' => Some(Tile::Empty { h: 1, v: 1 }),
                    b'#' => Some(Tile::Galaxy),
                    b'\n' => None,
                    _ => unreachable!(),
                })
                .collect(),
            width: input.trim().lines().next().unwrap().len(),
        }
    }

    fn expand(&mut self, count: usize) {
        for row in self.grid.chunks_mut(self.width) {
            if row.iter().all(|t| matches!(t, Tile::Empty { .. })) {
                row.iter_mut().for_each(|v| match v {
                    Tile::Empty { h, .. } => *h = count,
                    _ => unreachable!(),
                })
            }
        }

        for x in 0..self.width {
            let col_it = (x..self.grid.len()).step_by(self.width);

            if col_it
                .clone()
                .all(|i| matches!(&self.grid[i], Tile::Empty { .. }))
            {
                col_it.for_each(|i| match &mut self.grid[i] {
                    Tile::Empty { v, .. } => *v = count,
                    _ => unreachable!(),
                })
            }
        }
    }

    fn iter_galaxies(&self) -> GalaxyIter {
        let mut rows = self.grid.chunks_exact(self.width);
        GalaxyIter {
            cur_x: 0,
            cur_y: 0,
            y_add: 1,
            tiles: rows.next().unwrap_or(&[]).iter(),
            rows,
        }
    }
}

struct GalaxyIter<'a> {
    cur_x: usize,
    cur_y: usize,
    y_add: usize,
    tiles: std::slice::Iter<'a, Tile>,
    rows: std::slice::ChunksExact<'a, Tile>,
}

impl<'a> Iterator for GalaxyIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tiles.next() {
                Some(Tile::Galaxy) => {
                    let x = self.cur_x;
                    self.cur_x += 1;
                    return Some(Pos { x, y: self.cur_y });
                }
                Some(&Tile::Empty { h, v }) => {
                    self.y_add = self.y_add.max(h);
                    self.cur_x += v;
                }
                None => {
                    self.tiles = self.rows.next()?.iter();
                    self.cur_y += self.y_add;
                    self.cur_x = 0;
                    self.y_add = 1;
                }
            }
        }
    }
}

struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn ortho_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn expand_and_sum_distances(data: &str, count: usize) -> usize {
    let mut universe = Universe::from_input(data);

    universe.expand(count);

    let galaxies: Vec<_> = universe.iter_galaxies().collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(move |b| a.ortho_distance(&b))
        })
        .sum()
}

fn part1(data: &str) -> usize {
    expand_and_sum_distances(data, 2)
}

fn part2(data: &str) -> usize {
    expand_and_sum_distances(data, 1_000_000)
}

fn main() {
    println!("Day 11");
    let data = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn day11_part1() {
        assert_eq!(part1(DATA1), 374);
    }

    #[test]
    fn day11_part2_10() {
        assert_eq!(expand_and_sum_distances(DATA1, 10), 1030);
    }

    #[test]
    fn day11_part2_100() {
        assert_eq!(expand_and_sum_distances(DATA1, 100), 8410);
    }
}
