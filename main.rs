#![feature(destructuring_assignment)]

use rand::prelude::*;
use std::convert::TryInto;


fn distribute_cards() -> (Vec<u32>, Vec<u32>) {
    // Ace (15), King (14), Queen (12), Valete (11), 10, 9, 8, 7
    // Done 4 times: Hearth, Tiles, Clovers, Pikes
    let mut cards: Vec<u32> = Vec::new();
    for _ in 1..5 {
        // cards = (7..16).collect();
        for card in 7..16 {
            cards.push(card);
        }
    }
    println!("Cards: {:?}", cards);
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);
    println!("Cards: {:?}", cards);

    // Split between players
    (cards.split_off(cards.len()/2), cards)
}

fn count_aces(player: &[u32]) -> u32 {
    return player.iter().filter(|&card| *card == 15).count().try_into().unwrap()
}

enum PlayResult {
    PlayerWonHand(u32),
    PlayerOutOfCards(u32),
}

fn play_once (player1: &mut Vec<u32>, player2: &mut Vec<u32>, stack: &mut Vec<u32>) -> PlayResult {
    let card1 = match player1.pop() {
        Some(card) => card,
        None => return PlayResult::PlayerOutOfCards(1),
    };
    let card2 = match player2.pop() {
        Some(card) => card,
        None => return PlayResult::PlayerOutOfCards(2),
    };
    print!("{}  {}", card1, card2);
    // stack.insert(0, card1);
    // stack.insert(0, card2);
    if card1 > card2 {
        player1.insert(0, card1);
        player1.insert(0, card2);
        for card in stack {
            player1.insert(0, *card);
        }
        // stack.clear();
        print!("\t{}  {}", player1.len(), player2.len());
        return PlayResult::PlayerWonHand(1)
    } else if card1 < card2 {
        player2.insert(0, card2);
        player2.insert(0, card1);
        for card in stack {
            player2.insert(0, *card);
        }
        // stack.clear();
        print!("\t{}  {}", player1.len(), player2.len());
        return PlayResult::PlayerWonHand(2)
    }
    else {
        println!("  /");
        let hidden1 = match player1.pop() {
            Some(card) => card,
            None => return PlayResult::PlayerOutOfCards(1),
        };
        let hidden2 = match player2.pop() {
            Some(card) => card,
            None => return PlayResult::PlayerOutOfCards(2),
        };
        for card in vec![card1, hidden1, card2, hidden2] {
            stack.push(card);
        }
        return play_once(player1, player2, stack)
    }
}

fn main() {
    let mut player1: Vec<u32>;
    let mut player2: Vec<u32>;
    // let mut stack: Vec<u32> = Vec::new();
    
    println!("Battle Card Game Simulator");

    (player1, player2) = distribute_cards();
    println!("Player1: {:?}", player1);
    println!("Player2: {:?}", player2);

    loop {
        // match play_once(&mut player1, &mut player2, &mut stack) {
        match play_once(&mut player1, &mut player2, &mut Vec::new()) {
            PlayResult::PlayerWonHand(_winner) => {
                // println!(" Player {} won!", winner)
                println!("\t{}\u{1F0A1}\t{}\u{1F0D1}", count_aces(&player1), count_aces(&player2))
            }
            PlayResult::PlayerOutOfCards(player) => {
                println!(" Player {} ran out of cards!", player);
                return;
            }
        }
    }
}