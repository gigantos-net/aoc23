#[derive(Debug)]
enum Cell {
    Num { digits: u8, value: u16 },
    NumPart,
    Gear,
    Empty,
    Part,
}

impl Cell {
    fn is_part(&self) -> bool {
        match self {
            Self::Num { .. } | Self::NumPart | Self::Empty => false,
            Self::Gear | Self::Part => true,
        }
    }
}

struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn from_input(data: &str) -> Self {
        let mut ret = Self {
            grid: data
                .trim()
                .lines()
                .map(|line| -> Vec<Cell> {
                    line.as_bytes()
                        .iter()
                        .map(|c| match c {
                            b'0'..=b'9' => Cell::Num {
                                digits: 1,
                                value: u16::from(c - b'0'),
                            },
                            b'*' => Cell::Gear,
                            b'.' => Cell::Empty,
                            _ => Cell::Part,
                        })
                        .collect()
                })
                .collect(),
        };

        for row in ret.grid.iter_mut() {
            for i in 0..row.len() - 1 {
                if let &[Cell::Num {
                    digits: adig,
                    value: aval,
                }, Cell::Num {
                    digits: bdig,
                    value: bval,
                }] = &row[i..i + 2]
                {
                    row[i] = Cell::NumPart;
                    row[i + 1] = Cell::Num {
                        digits: adig + bdig,
                        value: (aval * 10) + bval,
                    };
                }
            }
        }

        ret
    }
}

fn part1(data: &str) -> usize {
    let grid = Grid::from_input(data);
    let grid = grid.grid.as_slice();

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| match cell {
                &Cell::Num {
                    digits,
                    value: value @ 1..,
                } => {
                    let range = x.saturating_sub(usize::from(digits))..(x + 2).min(row.len());
                    for y in y.saturating_sub(1)..(y + 2).min(grid.len()) {
                        if grid[y][range.clone()].iter().any(|v| v.is_part()) {
                            return value;
                        }
                    }
                    0
                }
                _ => 0,
            })
        })
        .map(usize::from)
        .sum()
}

fn part2(data: &str) -> usize {
    let grid = Grid::from_input(data);
    let grid = grid.grid.as_slice();

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| match cell {
                Cell::Gear => {
                    let range = x.saturating_sub(1)..(x + 2).min(row.len());
                    let mut nums = Vec::with_capacity(2);
                    for y in y.saturating_sub(1)..(y + 2).min(grid.len()) {
                        let mut last_was_numpart = true;
                        for cell in grid[y][range.clone()].iter() {
                            last_was_numpart = false;
                            match *cell {
                                Cell::Num { value, .. } => nums.push(usize::from(value)),
                                Cell::NumPart => last_was_numpart = true,
                                _ => (),
                            }
                        }
                        if last_was_numpart {
                            'get_num: for cell in &grid[y][range.end..] {
                                match cell {
                                    &Cell::Num { value, .. } => {
                                        nums.push(usize::from(value));
                                        break 'get_num;
                                    }
                                    Cell::NumPart => continue,
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                    if let Some(&[a, b]) = nums.get(0..2) {
                        a * b
                    } else {
                        0
                    }
                }
                _ => 0,
            })
        })
        .sum()
}

fn main() {
    println!("Day 2");
    let data = std::fs::read_to_string("input/day03.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn day03_part1() {
        assert_eq!(part1(PART1), 4361);
    }

    #[test]
    fn day03_part2() {
        assert_eq!(part2(PART1), 467835);
    }
}
