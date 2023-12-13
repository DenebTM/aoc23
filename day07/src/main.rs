use std::{io, iter::zip};

use hand::Hand;
use hand_alt::Hand as HandAlt;

mod card;
mod card_alt;
mod hand;
mod hand_alt;
mod hand_type;

fn main() {
    let mut hands_bets: Vec<(Hand, u64)> = Vec::new();
    let mut hands_bets_alt: Vec<(HandAlt, u64)> = Vec::new();

    let stdin = io::stdin();
    let mut buf = String::new();
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        let (hand_str, bet_str) = buf.split_at(buf.find(' ').unwrap());

        let hand = Hand::try_from(hand_str).unwrap();
        let hand_alt = HandAlt::try_from(hand_str).unwrap();
        let bet = bet_str.trim().parse::<u64>().unwrap();

        hands_bets.push((hand, bet));
        hands_bets_alt.push((hand_alt, bet));

        buf.clear();
    }

    hands_bets.sort_by(|l, r| l.0.cmp(&r.0));
    hands_bets_alt.sort_by(|l, r| l.0.cmp(&r.0));

    let (sum, sum_alt): (u64, u64) = zip(hands_bets.iter(), hands_bets_alt.iter())
        .enumerate()
        .map(|(index, ((hand, bet), (hand_alt, bet_alt)))| {
            let rank = (index + 1) as u64;
            let score = rank * bet;
            let score_alt = rank * bet_alt;

            println!("rank {}:", rank);
            println!(
                "- part1: {:?} - cards: {}; bet: {:>3} -> {}",
                hand.get_type(),
                hand,
                bet,
                score
            );
            println!(
                "- part2: {:?} - cards: {}; bet: {:>3} -> {}\n",
                hand_alt.get_type(),
                hand_alt,
                bet_alt,
                score_alt
            );

            (score, score_alt)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("part1: {}\npart2: {}", sum, sum_alt);
}
