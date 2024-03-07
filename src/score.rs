use super::logic::RoundResult;

pub trait Scoring {
    fn get_score(&self) -> [i32; 3];
    fn update_score(&mut self, round_result: &[RoundResult; 3]);
}

pub struct NormalScoring {
    score: [i32; 3],
}

impl NormalScoring {
    pub fn new() -> Self {
        NormalScoring { score: [0; 3] }
    }
}

impl Scoring for NormalScoring {
    fn get_score(&self) -> [i32; 3] {
        self.score
    }

    fn update_score(&mut self, round_result: &[RoundResult; 3]) {
        for i in 0..3 {
            match round_result[i] {
                RoundResult::Win => self.score[i] += 1,
                RoundResult::Tie => (),
                RoundResult::Lose => self.score[i] -= 1,
            }
        }
    }
}

pub struct ComboScoring {
    score: [i32; 3],
    combo: [u32; 3],
}

impl ComboScoring {
    pub fn new() -> Self {
        ComboScoring {
            score: [0; 3],
            combo: [0; 3],
        }
    }
}

impl Scoring for ComboScoring {
    fn get_score(&self) -> [i32; 3] {
        self.score
    }

    fn update_score(&mut self, round_result: &[RoundResult; 3]) {
        for i in 0..3 {
            match round_result[i] {
                RoundResult::Win => {
                    self.combo[i] += 1;
                    self.score[i] += self.combo[i] as i32;
                }
                RoundResult::Tie => (),
                RoundResult::Lose => self.combo[i] = 0,
            }
        }
    }
}
