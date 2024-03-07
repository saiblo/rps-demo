mod io_types;
mod logic;
mod protocol;
mod score;

#[cfg(test)]
mod test;

use io_types::{Display, InitData, Request, Response};
use logic::{judge_round, Gesture, RoundResult};
use protocol::{AgentStatus, FinishMessage, FinishOutput, InitInput, RoundInput, RoundOutput};
use score::{ComboScoring, NormalScoring, Scoring};
use std::collections::HashMap;

const MAX_ROUND: u32 = 512;

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();

    // Init
    let mut init_str = String::new();
    stdin.read_line(&mut init_str)?;
    let init: InitInput<InitData> = serde_json::from_str(&init_str).unwrap();
    let mut scoring: Box<dyn Scoring> = if init.initdata.use_combo_scoring {
        Box::new(ComboScoring::new())
    } else {
        Box::new(NormalScoring::new())
    };

    // Round output and input
    let mut last_gestures = [Gesture::Rock; 3];
    let mut last_result = [RoundResult::Tie; 3];
    for _round in 0..MAX_ROUND {
        // Prepare output
        let request = Request {
            last_gestures,
            last_result,
            total_scores: scoring.get_score(),
        };
        let mut output_content = HashMap::new();
        output_content.insert("0".to_owned(), request.clone());
        output_content.insert("1".to_owned(), request.clone());
        output_content.insert("2".to_owned(), request);
        let display = Display {
            last_gestures,
            last_result,
            total_scores: scoring.get_score(),
        };
        let round_output = RoundOutput {
            content: output_content.into(),
            display: display.clone(),
        };
        println!("{}", serde_json::to_string(&round_output).unwrap());

        // Parse input
        let mut input_str = String::new();
        stdin.read_line(&mut input_str)?;
        let round_input: RoundInput<Response> = serde_json::from_str(&input_str).unwrap();
        let responses = round_input.log.0;
        // Check AI status
        let mut ok = true;
        let mut finish_message = HashMap::<String, FinishMessage>::new();
        responses.iter().for_each(|(k, v)| {
            if v.verdict != AgentStatus::OK {
                finish_message.insert(
                    k.to_owned(),
                    FinishMessage {
                        score: 0.,
                        state: "FAIL".to_owned(),
                    },
                );
                ok = false;
            }
        });
        // AI error
        if !ok {
            let finish = FinishOutput {
                content: finish_message,
                display,
            };
            println!("{}", serde_json::to_string(&finish).unwrap());
            return Ok(());
        }
        // Get gestures
        last_gestures = ["0", "1", "2"].map(|k| responses.get(k).unwrap().response.gesture);
        // Judge
        last_result = judge_round(last_gestures);
        // Scoring
        scoring.update_score(&last_result);
    }

    // Finish
    let score = scoring.get_score();
    let mut finish_message = HashMap::<String, FinishMessage>::new();
    finish_message.insert(
        "0".to_owned(),
        FinishMessage {
            score: score[0] as f32,
            state: "OK".to_owned(),
        },
    );
    finish_message.insert(
        "1".to_owned(),
        FinishMessage {
            score: score[1] as f32,
            state: "OK".to_owned(),
        },
    );
    finish_message.insert(
        "2".to_owned(),
        FinishMessage {
            score: score[2] as f32,
            state: "OK".to_owned(),
        },
    );
    let finish = FinishOutput {
        content: finish_message.into(),
        display: Display {
            last_gestures,
            last_result,
            total_scores: scoring.get_score(),
        },
    };
    println!("{}", serde_json::to_string(&finish).unwrap());

    // Dead loop: wait to be killed by judger
    let mut i = 0;
    loop {
        i = 0;
    }
}
