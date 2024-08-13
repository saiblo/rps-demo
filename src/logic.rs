use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum RoundResult {
    Win,
    Tie,
    Lose,
}

fn judge2(a: Gesture, b: Gesture) -> RoundResult {
    match a {
        Gesture::Rock => match b {
            Gesture::Rock => RoundResult::Tie,
            Gesture::Paper => RoundResult::Lose,
            Gesture::Scissors => RoundResult::Win,
        },
        Gesture::Paper => match b {
            Gesture::Rock => RoundResult::Win,
            Gesture::Paper => RoundResult::Tie,
            Gesture::Scissors => RoundResult::Lose,
        },
        Gesture::Scissors => match b {
            Gesture::Paper => RoundResult::Win,
            Gesture::Rock => RoundResult::Lose,
            Gesture::Scissors => RoundResult::Tie,
        },
    }
}

fn judge3(me: Gesture, op1: Gesture, op2: Gesture) -> RoundResult {
    let mut score = 0;
    [op1, op2].iter().for_each(|op| match judge2(me, *op) {
        RoundResult::Win => score += 1,
        RoundResult::Tie => (),
        RoundResult::Lose => score -= 1,
    });
    if score > 0 {
        RoundResult::Win
    } else if score == 0 {
        RoundResult::Tie
    } else {
        RoundResult::Lose
    }
}

pub fn judge_round(players: [Gesture; 3]) -> [RoundResult; 3] {
    [
        judge3(players[0], players[1], players[2]),
        judge3(players[1], players[0], players[2]),
        judge3(players[2], players[0], players[1]),
    ]
}
