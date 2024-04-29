use std::collections::HashMap;

#[derive(Debug)]
pub struct MatchScores {
    pub scores: Vec<MatchScore>,
}
impl MatchScores {
    pub fn new() -> Self {
        MatchScores { scores: Vec::new() }
    }

    pub fn push(&mut self, score: MatchScore) {
        self.scores.push(score);
    }
}

#[derive(Debug)]
pub struct MatchScore {
    pub total_kills: i32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, i32>,
}
impl MatchScore {
    pub fn new() -> Self {
        MatchScore {
            total_kills: 0,
            players: Vec::new(),
            kills: HashMap::new(),
            kills_by_means: HashMap::new(),
        }
    }

    pub fn increase_kill_by_means(&mut self, mean: &str) {
        match self.kills_by_means.get(mean) {
            Some(k) => self.kills_by_means.insert(mean.to_string(), k + 1),
            None => self.kills_by_means.insert(mean.to_string(), 1),
        };
    }
}
