use common::load_groups;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn main() {
    let (player1, player2) = {
        let input: Vec<Player> = load_groups("input/day22.txt");
        let mut iter = input.into_iter();
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let winning_score = play_standard(&mut player1.clone(), &mut player2.clone());
    println!("Part 1: {}", winning_score);

    let winning_score = play_recursive(HashSet::new(), &mut player1.clone(), &mut player2.clone());
    println!("Part 2: {}", winning_score.score);
}

fn play_standard(player1: &mut Player, player2: &mut Player) -> usize {
    loop {
        let p1card = if let Some(c) = player1.draw() {
            c
        } else {
            break;
        };
        let p2card = if let Some(d) = player2.draw() {
            d
        } else {
            break;
        };
        if p1card > p2card {
            player1.claim(p1card, p2card)
        } else {
            player2.claim(p2card, p1card)
        }
    }

    max(player1.score(), player2.score())
}

#[derive(Copy, Clone, Debug)]
enum PlayerId {
    Player1,
    Player2,
}

#[derive(Copy, Clone, Debug)]
struct RoundResult {
    winner: PlayerId,
    score: usize,
}

fn play_recursive<'a>(
    mut prev_rounds: HashSet<(Vec<u8>, Vec<u8>)>,
    mut player1: &'a mut Player,
    mut player2: &'a mut Player,
) -> RoundResult {
    if player1.deck.len() == 0 {
        return RoundResult {
            winner: PlayerId::Player2,
            score: player2.score(),
        };
    } else if player2.deck.len() == 0 {
        return RoundResult {
            winner: PlayerId::Player1,
            score: player1.score(),
        };
    }

    let key = (
        player1.deck.iter().map(|i| *i).collect(),
        player2.deck.iter().map(|i| *i).collect(),
    );
    if prev_rounds.contains(&key) {
        return RoundResult {
            winner: PlayerId::Player1,
            score: player1.score(),
        };
    }

    let (card1, card2) = (
        player1.draw().unwrap() as usize,
        player2.draw().unwrap() as usize,
    );

    let winner = if player1.deck.len() >= card1 && player2.deck.len() >= card2 {
        // can play a recursive subgame
        let subgame_result = play_recursive(
            HashSet::new(),
            &mut player1.copy_n(card1).unwrap(),
            &mut player2.copy_n(card2).unwrap(),
        );

        match subgame_result.winner {
            PlayerId::Player1 => &mut player1,
            PlayerId::Player2 => &mut player2,
        }
    } else if card1 > card2 {
        &mut player1
    } else {
        &mut player2
    };

    let (winning_card, losing_card) = match winner.id {
        PlayerId::Player1 => (card1 as u8, card2 as u8),
        PlayerId::Player2 => (card2 as u8, card1 as u8),
    };

    winner.claim(winning_card, losing_card);

    prev_rounds.insert(key);

    play_recursive(prev_rounds, player1, player2)
}

#[derive(Debug, Clone)]
struct Player {
    id: PlayerId,
    deck: VecDeque<u8>,
}

impl Player {
    fn draw(&mut self) -> Option<u8> {
        self.deck.pop_front()
    }

    fn claim(&mut self, card1: u8, card2: u8) {
        self.deck.push_back(card1);
        self.deck.push_back(card2);
    }

    fn score(&self) -> usize {
        if self.deck.is_empty() {
            return 0;
        }
        let size = self.deck.len();
        self.deck
            .iter()
            .enumerate()
            .map(|(i, x)| (size - i) * (*x as usize))
            .sum()
    }

    fn copy_n(&self, n: usize) -> Option<Self> {
        if n > self.deck.len() {
            return None;
        }

        Some(Player {
            id: self.id,
            deck: self.deck.iter().take(n).map(|i| *i).collect(),
        })
    }
}

impl FromStr for Player {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let player_id_s = String::from(
            lines
                .next()
                .unwrap()
                .replace("Player ", "")
                .replace(":", "")
                .trim(),
        );
        let id = match u8::from_str(&player_id_s).map_err(|e| format!("{}", e)) {
            Ok(1) => Ok(PlayerId::Player1),
            Ok(2) => Ok(PlayerId::Player2),
            _ => Err(format!("Unexpected player id: '{}'", player_id_s)),
        }?;
        let deck: VecDeque<u8> = lines
            .map(|line| u8::from_str(line).map_err(|e| format!("{}", e)))
            .collect::<Result<_, _>>()?;
        Ok(Player { id, deck })
    }
}
