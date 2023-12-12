#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Card(u8);

static CARDS: [Card; 256] = {
    let mut val = [Card(0); 256];
    val[b'2' as usize] = Card(2);
    val[b'3' as usize] = Card(3);
    val[b'4' as usize] = Card(4);
    val[b'5' as usize] = Card(5);
    val[b'6' as usize] = Card(6);
    val[b'7' as usize] = Card(7);
    val[b'8' as usize] = Card(8);
    val[b'9' as usize] = Card(9);
    val[b'T' as usize] = Card(10);
    val[b'J' as usize] = Card(11);
    val[b'Q' as usize] = Card(12);
    val[b'K' as usize] = Card(13);
    val[b'A' as usize] = Card(14);
    val
};

static CARDS2: [Card; 256] = {
    let mut val = CARDS;
    val[b'J' as usize] = Card(1);
    val
};

fn calc_score(mut cards: [Card; 5]) -> usize {
    cards.sort();

    let mut counts = [0; 6];
    {
        let mut it = cards.into_iter().filter(|&c| c != Card(1));
        if let Some(mut prev_card) = it.next() {
            let mut count = 1;

            for card in it {
                if card == prev_card {
                    count += 1;
                } else {
                    counts[count] += 1;
                    count = 1;
                }
                prev_card = card;
            }

            counts[count] += 1;
        }
    }
    let jokers = cards.into_iter().filter(|&c| c == Card(1)).count();

    if let Some(i) = counts.iter().rposition(|&c| c > 0) {
        counts[i] -= 1;
        counts[i + jokers] += 1;
    } else {
        // All jokers
        counts[5] += 1;
    }

    if counts[5] == 1 {
        7
    } else if counts[4] == 1 {
        6
    } else if counts[3] == 1 && counts[2] == 1 {
        5
    } else if counts[3] == 1 {
        4
    } else if counts[2] == 2 {
        3
    } else if counts[2] == 1 {
        2
    } else {
        1
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand {
    score: usize,
    cards: [Card; 5],
    bet: usize,
}

impl Hand {
    fn from_input(input: &str, card_map: &[Card; 256]) -> Self {
        let (hand, bet) = input.split_once(' ').unwrap();
        let cards = <[u8; 5]>::try_from(hand.as_bytes())
            .unwrap()
            .map(|v| card_map[v as usize]);

        let score = calc_score(cards);

        Self {
            score,
            cards,
            bet: bet.parse().unwrap(),
        }
    }
}

fn calc_card_winnings(data: &str, card_map: &[Card; 256]) -> usize {
    let mut hands: Vec<_> = data
        .trim()
        .lines()
        .map(|v| Hand::from_input(v, card_map))
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1))
        .sum()
}

fn part1(data: &str) -> usize {
    calc_card_winnings(data, &CARDS)
}

fn part2(data: &str) -> usize {
    calc_card_winnings(data, &CARDS2)
}

fn main() {
    println!("Day 7");
    let data = std::fs::read_to_string("input/day07.txt").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn day07_part1() {
        assert_eq!(part1(PART1), 6440);
    }

    #[test]
    fn day07_part2() {
        assert_eq!(part2(PART1), 5905);
    }

    #[test]
    fn day07_extra() {
        assert_eq!(part2("JJJJJ 1337\n22223 326"), 3000);
    }
}
