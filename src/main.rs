mod io_types;
mod logic;
mod score;

#[cfg(test)]
mod test;

extern crate uniform_logic_rs;

use io_types::{Display, InitData, Request, Response};
use logic::{judge_round, Gesture, RoundResult};
use score::{ComboScoring, NormalScoring, Scoring};
use uniform_logic_rs::*;

const MAX_ROUND: u32 = 512;

fn main() -> std::io::Result<()> {
    // Init
    let init = receive_init_message::<InitData>().unwrap();
    let mut scoring: Box<dyn Scoring> = if init.use_combo_scoring {
        Box::new(ComboScoring::new())
    } else {
        Box::new(NormalScoring::new())
    };

    // Round output and input
    let mut last_gestures = [Gesture::Rock; 3];
    let mut last_result = [RoundResult::Tie; 3];

    for _round in 0..MAX_ROUND {
        let total_scores = scoring.get_score();
        // Round output
        let request = Request {
            last_gestures,
            last_result,
            total_scores,
        };
        let display = Display {
            last_gestures,
            last_result,
            total_scores,
        };
        let mut sender = RoundMessageSender::<Request, Display>::new();
        sender.send_agent("0".to_owned(), request.clone());
        sender.send_agent("1".to_owned(), request.clone());
        sender.send_agent("2".to_owned(), request.clone());
        sender.send_display(display.clone());
        sender.end().unwrap();

        // Round input
        let responses = recieve_round_message::<Response>().unwrap();
        // AI error
        if responses
            .values()
            .any(|res| res.verdict != AgentVerdict::OK)
        {
            let mut sender = FinishMessageSender::<Display>::new();
            for (name, response) in responses {
                if response.verdict != AgentVerdict::OK {
                    sender.send_agent(name, 0., "FAIL".to_owned());
                } else {
                    let id: usize = str::parse(&name).unwrap();
                    sender.send_agent(name, total_scores[id] as f32, "OK".to_owned());
                }
            }
            sender.send_display(display);
            sender.end().unwrap();
            return Ok(());
        }
        // Get gestures
        for i in 0..3 {
            let names = ["0", "1", "2"];
            last_gestures[i] = responses.get(names[i]).unwrap().response.gesture;
        }
        // Judge
        last_result = judge_round(last_gestures);
        // Scoring
        scoring.update_score(&last_result);
    }

    // Finish
    let score = scoring.get_score();
    let display = Display {
        last_gestures,
        last_result,
        total_scores: score,
    };
    let mut sender = FinishMessageSender::<Display>::new();
    sender.send_agent("0".to_owned(), score[0] as f32, "OK".to_owned());
    sender.send_agent("1".to_owned(), score[1] as f32, "OK".to_owned());
    sender.send_agent("2".to_owned(), score[2] as f32, "OK".to_owned());
    sender.send_display(display);
    sender.end().unwrap();

    // Dead loop: wait to be killed by judger
    loop {}
}
