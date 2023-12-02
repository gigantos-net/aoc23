#[derive(Default, Clone)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_game(line: &str) -> (usize, impl Iterator<Item = GameSet> + '_) {
    let (game_num, results) = line
        .strip_prefix("Game ")
        .unwrap()
        .split_once(": ")
        .unwrap();
    (
        game_num.parse().unwrap(),
        results.split("; ").map(|set| {
            let mut ret = GameSet::default();
            for res in set.split(", ") {
                let (count, color) = res.split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();
                match color {
                    "red" => ret.red += count,
                    "green" => ret.green += count,
                    "blue" => ret.blue += count,
                    _ => panic!("Invalid color {color:?}"),
                }
            }
            ret
        }),
    )
}

fn part1(data: &str) -> usize {
    let bag = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    data.trim()
        .lines()
        .filter_map(|l| -> Option<usize> {
            let (game_num, sets) = parse_game(l);
            for GameSet { red, green, blue } in sets {
                if red > bag.red || green > bag.green || blue > bag.blue {
                    return None;
                }
            }

            Some(game_num)
        })
        .sum()
}

fn part2(data: &str) -> usize {
    data.trim()
        .lines()
        .filter_map(|l| -> Option<usize> {
            let (_, sets) = parse_game(l);
            let mut min_game = GameSet::default();
            for GameSet { red, green, blue } in sets {
                min_game.red = red.max(min_game.red);
                min_game.green = green.max(min_game.green);
                min_game.blue = blue.max(min_game.blue);
            }

            Some(min_game.red * min_game.green * min_game.blue)
        })
        .sum()
}

fn main() {
    println!("Day 2");
    let data = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn day02_part1() {
        assert_eq!(part1(PART1), 8);
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(PART1), 2286);
    }
}
