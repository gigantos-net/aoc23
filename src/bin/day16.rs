const MIRROR_F: u8 = 0b0100;
const MIRROR_B: u8 = 0b0101;
const SPLIT_V: u8 = 0b0110;
const SPLIT_H: u8 = 0b0111;
const TYPE_MASK: u8 = 0b0111;

struct Map {
    data: Vec<u8>,
    width: usize,
}

impl Map {
    fn from_input(data: &str) -> Self {
        let data = data.trim();
        let width = data.lines().next().unwrap().len();

        Self {
            data: data
                .lines()
                .flat_map(|v| v.as_bytes())
                .map(|b| match b {
                    b'.' => 0,
                    b'/' => MIRROR_F,
                    b'\\' => MIRROR_B,
                    b'|' => SPLIT_V,
                    b'-' => SPLIT_H,
                    _ => unreachable!(),
                })
                .collect(),
            width,
        }
    }

    fn trace_beam(&mut self, mut pos: Pos, mut dir: Dir) -> usize {
        let mut ret = 0;
        while let Some(cur) = pos.index(self.width).and_then(|i| self.data.get_mut(i)) {
            let cur_flags = dir.as_flags();

            if (*cur & cur_flags) == cur_flags {
                return ret;
            }

            if *cur <= 0b1111 {
                ret += 1;
            }

            *cur |= cur_flags;

            match *cur & TYPE_MASK {
                0 => (),
                MIRROR_F => dir.neg_swap(),
                MIRROR_B => dir.swap(),
                SPLIT_V => {
                    if matches!(dir, Dir::L | Dir::R) {
                        dir.swap();
                        ret += self.trace_beam(pos + dir, dir);
                        dir.neg();
                    }
                }
                SPLIT_H => {
                    if matches!(dir, Dir::U | Dir::D) {
                        dir.swap();
                        ret += self.trace_beam(pos + dir, dir);
                        dir.neg();
                    }
                }
                _ => unreachable!(),
            }
            pos = pos + dir;
        }
        ret
    }

    fn reset(&mut self) {
        self.data.iter_mut().for_each(|v| *v &= 0b1111);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn index(self, width: usize) -> Option<usize> {
        let x = usize::try_from(self.x).ok()?;
        if x < width {
            let y = usize::try_from(self.y).ok()?;
            Some((y * width) + x)
        } else {
            None
        }
    }

    fn new(x: usize, y: usize) -> Pos {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
enum Dir {
    L = 0b00010000,
    R = 0b00100000,
    U = 0b01000000,
    D = 0b10000000,
}

impl Dir {
    fn swap(&mut self) {
        *self = match self {
            Self::L => Self::U,
            Self::R => Self::D,
            Self::U => Self::L,
            Self::D => Self::R,
        }
    }

    fn neg(&mut self) {
        *self = match self {
            Self::L => Self::R,
            Self::R => Self::L,
            Self::U => Self::D,
            Self::D => Self::U,
        }
    }

    fn neg_swap(&mut self) {
        *self = match self {
            Self::L => Self::D,
            Self::R => Self::U,
            Self::U => Self::R,
            Self::D => Self::L,
        }
    }

    fn as_flags(self) -> u8 {
        self as u8
    }
}

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::L => self.x -= 1,
            Dir::R => self.x += 1,
            Dir::U => self.y -= 1,
            Dir::D => self.y += 1,
        }
        self
    }
}

fn part1(data: &str) -> usize {
    let mut map = Map::from_input(data);
    map.trace_beam(Pos { x: 0, y: 0 }, Dir::R)
}

fn part2(data: &str) -> usize {
    let mut map = Map::from_input(data);
    let height = map.data.len() / map.width;
    let mut ret = 0;

    for x in 0..map.width {
        ret = ret.max(map.trace_beam(Pos::new(x, 0), Dir::D));
        map.reset();
        ret = ret.max(map.trace_beam(Pos::new(x, height - 1), Dir::U));
        map.reset();
    }

    for y in 0..height {
        ret = ret.max(map.trace_beam(Pos::new(0, y), Dir::R));
        map.reset();
        ret = ret.max(map.trace_beam(Pos::new(map.width - 1, y), Dir::L));
        map.reset();
    }

    ret
}

fn main() {
    println!("Day 16");
    let data = std::fs::read_to_string("input/day16.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn day16_part1() {
        assert_eq!(part1(DATA1), 46);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(part2(DATA1), 51);
    }
}
