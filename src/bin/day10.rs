const PRINT_GRID: bool = false;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pipe(u8);

impl Pipe {
    const START: Self = Self(0b1111);
    const NORTH: Self = Self(0b0001);
    const SOUTH: Self = Self(0b0010);
    const EAST: Self = Self(0b0100);
    const WEST: Self = Self(0b1000);
    const NO_PIPE: Self = Self(0b0000);

    fn from_input(input: u8) -> Self {
        match input {
            b'S' => Self::START,
            b'|' => Self(Self::NORTH.0 | Self::SOUTH.0),
            b'-' => Self(Self::EAST.0 | Self::WEST.0),
            b'L' => Self(Self::NORTH.0 | Self::EAST.0),
            b'J' => Self(Self::NORTH.0 | Self::WEST.0),
            b'7' => Self(Self::SOUTH.0 | Self::WEST.0),
            b'F' => Self(Self::SOUTH.0 | Self::EAST.0),
            b'.' => Self(0),
            unk => unreachable!("{}", char::from(unk)),
        }
    }

    fn can_move_west(self) -> bool {
        (self.0 & Self::WEST.0) != 0
    }

    fn can_move_north(self) -> bool {
        (self.0 & Self::NORTH.0) != 0
    }

    fn can_move_east(self) -> bool {
        (self.0 & Self::EAST.0) != 0
    }

    fn can_move_south(self) -> bool {
        (self.0 & Self::SOUTH.0) != 0
    }

    fn move_trough(mut self, from: Pipe, pos: Pos) -> (Self, Pos) {
        assert_eq!(from.0.count_ones(), 1);
        assert_eq!(self.0 & from.0, from.0);
        self.0 &= !from.0;
        match self {
            Pipe::NORTH => (Pipe::SOUTH, pos.move_north()),
            Pipe::SOUTH => (Pipe::NORTH, pos.move_south()),
            Pipe::EAST => (Pipe::WEST, pos.move_east()),
            Pipe::WEST => (Pipe::EAST, pos.move_west()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }

    fn index(self, stride: usize) -> Option<usize> {
        let x = usize::try_from(self.x).ok()?;
        if x >= stride {
            return None;
        }
        let y = usize::try_from(self.y).ok()?;
        Some((y * stride) + x)
    }

    fn move_east(mut self) -> Pos {
        self.x += 1;
        self
    }

    fn move_north(mut self) -> Pos {
        self.y -= 1;
        self
    }

    fn move_south(mut self) -> Pos {
        self.y += 1;
        self
    }

    fn move_west(mut self) -> Pos {
        self.x -= 1;
        self
    }

    fn scale_up(&self, mul: isize) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
        }
    }
}

struct Walker {
    image: FilledGrid,
    total_steps: usize,
}

struct Grid {
    data: Vec<Pipe>,
    width: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        Self {
            data: input
                .trim()
                .lines()
                .flat_map(|l| l.as_bytes().iter().copied())
                .map(Pipe::from_input)
                .collect(),
            width: input.trim().lines().next().unwrap().len(),
        }
    }

    fn walk_grid(&self) -> Walker {
        let start_i = self.data.iter().position(|&v| v == Pipe::START).unwrap();

        let start = Pos::new(start_i % self.width, start_i / self.width);

        let mut start_pipe = Pipe(0);

        if self.get(start.move_east()).can_move_west() {
            start_pipe.0 |= Pipe::EAST.0;
        }

        if self.get(start.move_south()).can_move_north() {
            start_pipe.0 |= Pipe::SOUTH.0;
        }

        if start.x > 0 && self.get(start.move_west()).can_move_east() {
            start_pipe.0 |= Pipe::WEST.0;
        }

        if start.y > 0 && self.get(start.move_north()).can_move_south() {
            start_pipe.0 |= Pipe::NORTH.0;
        }

        let mut from = Pipe(0x80 >> start_pipe.0.leading_zeros());
        let mut pos = start;

        let mut image = FilledGrid::new(self.width, self.data.len() / self.width);
        let mut total_steps = 1;

        (from, pos) = start_pipe.move_trough(from, pos);
        image.set(start, b'S', start_pipe);

        loop {
            total_steps += 1;

            let pipe = self.get(pos);

            image.set(pos, b'*', pipe);

            (from, pos) = pipe.move_trough(from, pos);

            if pos == start {
                break;
            }
        }

        Walker { image, total_steps }
    }

    fn get(&self, pos: Pos) -> Pipe {
        pos.index(self.width)
            .and_then(|i| self.data.get(i).copied())
            .unwrap_or(Pipe::NO_PIPE)
    }
}

struct FilledGrid {
    data: Vec<u8>,
    width: usize,
    stride: usize,
}

impl FilledGrid {
    fn new(width: usize, height: usize) -> Self {
        let raw_width = 2 + (width * 3);
        let raw_height = 2 + (height * 3);
        Self {
            data: vec![b' '; raw_height * raw_width],
            width,
            stride: raw_width,
        }
    }

    fn set(&mut self, pos: Pos, value: u8, pipe: Pipe) {
        let mut base = pos.scale_up(3).index(self.stride).unwrap() + 1 + self.stride;

        //
        // Row 1
        //
        self.data[base..base + 3].fill(b'-');

        if pipe.can_move_north() {
            self.data[base + 1] = b'*';
        }

        base += self.stride;

        //
        // Row 2
        //
        assert_eq!(char::from(self.data[base + 1]), ' ', "{pos:?}");

        self.data[base..base + 3].fill(b'-');

        if pipe.can_move_west() {
            self.data[base] = b'*';
        }

        // The tile itself
        self.data[base + 1] = value;

        if pipe.can_move_east() {
            self.data[base + 2] = b'*';
        }

        base += self.stride;

        //
        // Row 3
        //
        self.data[base..base + 3].fill(b'-');

        if pipe.can_move_south() {
            self.data[base + 1] = b'*';
        }
    }

    fn try_set_raw(&mut self, pos: Pos) -> bool {
        match pos.index(self.stride).and_then(|i| self.data.get_mut(i)) {
            Some(v @ b' ') => {
                *v = b'#';
                true
            }
            Some(v @ b'-') => {
                *v = b'@';
                true
            }
            _ => false,
        }
    }

    fn is_set_raw(&self, pos: Pos) -> bool {
        if let Some(v) = pos.index(self.stride).and_then(|i| self.data.get(i)) {
            matches!(v, b' ' | b'-')
        } else {
            false
        }
    }

    fn flood_fill(&mut self, pos: Pos) {
        let mut pending = vec![(pos.x, pos.x, pos.y, 1)];

        if pos.y > 0 {
            pending.push((pos.x, pos.x, pos.y - 1, -1));
        }

        while let Some((mut x1, x2, y, dy)) = pending.pop() {
            let mut x = x1;
            if self.is_set_raw(Pos { x, y }) {
                while self.try_set_raw(Pos { x: x - 1, y }) {
                    x -= 1;
                }
                if x < x1 {
                    pending.push((x, x1 - 1, y - dy, -dy));
                }
            }

            while x1 <= x2 {
                while self.try_set_raw(Pos { x: x1, y }) {
                    x1 += 1;
                }

                if x1 > x {
                    pending.push((x, x1 - 1, y + dy, dy));
                }

                if x1 - 1 > x2 {
                    pending.push((x2 + 1, x1 - 1, y - dy, -dy));
                }

                x1 += 1;

                while x1 < x2 && !self.is_set_raw(Pos { x: x1, y }) {
                    x1 += 1;
                }

                x = x1;
            }
        }
    }

    fn print_grid(&self) {
        if !PRINT_GRID {
            return;
        }
        println!();
        if self.width < 10 {
            print!("     ");
            for i in 0..self.width {
                print!("{0}{0}{0}", i % 10);
            }
            println!();
        }
        for (i, line) in self.data[1..].chunks(self.stride).skip(1).enumerate() {
            println!("{:>3} |{}|", i / 3, std::str::from_utf8(&line).unwrap());
        }
        println!();
    }
}

fn part1(data: &str) -> usize {
    if PRINT_GRID {
        println!("\n{}\n", data.trim());
    }

    let grid = Grid::from_input(data);
    let res = grid.walk_grid();

    res.image.print_grid();

    (res.total_steps / 2) + (res.total_steps % 2)
}

fn part2(data: &str) -> usize {
    if PRINT_GRID {
        println!("\n{}\n", data.trim());
    }

    let grid = Grid::from_input(data);
    let mut res = grid.walk_grid();

    res.image.print_grid();

    res.image.flood_fill(Pos { x: 0, y: 0 });

    res.image.print_grid();

    res.image.data.iter().filter(|&&b| b == b' ').count() / 9
}

fn main() {
    println!("Day 10");
    let data = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    const DATA2: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    const DATA3: &str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    const DATA4: &str = r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

    const DATA5: &str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    #[test]
    fn day10_part1_data1() {
        assert_eq!(part1(DATA1), 4);
    }

    #[test]
    fn day10_part1_data2() {
        assert_eq!(part1(DATA2), 8);
    }

    #[test]
    fn day10_part2_data3() {
        assert_eq!(part2(DATA3), 4);
    }

    #[test]
    fn day10_part2_data4() {
        assert_eq!(part2(DATA4), 4);
    }

    #[test]
    fn day10_part2_data5() {
        assert_eq!(part2(DATA5), 8);
    }
}
