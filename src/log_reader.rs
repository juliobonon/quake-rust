use super::parsers::parse_game_lines;
use super::scores::match_score::MatchScores;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum GameLine {
    InitGame,
    ShutdownGame,
    Kill,
    Generic,
}
impl GameLine {
    fn from_string(line: &str) -> GameLine {
        if line.contains("InitGame") {
            GameLine::InitGame
        } else if line.contains("ShutdownGame") {
            GameLine::ShutdownGame
        } else if line.contains("Kill") {
            GameLine::Kill
        } else {
            GameLine::Generic
        }
    }
}

pub fn read_log_file(file: File) -> Result<MatchScores, Box<dyn Error>> {
    let reader = BufReader::new(file);
    let mut match_score_lines: Vec<String> = Vec::new();
    let mut match_scores = MatchScores::new();
    let mut match_started = false;

    for line in reader.lines() {
        let line = line?;
        match GameLine::from_string(&line) {
            GameLine::InitGame => {
                if match_started {
                    match_scores.push(parse_game_lines(&match_score_lines));
                }
                match_score_lines.clear();
                match_started = true;
            }
            GameLine::ShutdownGame => {
                match_scores.push(parse_game_lines(&match_score_lines));
                match_score_lines.clear();
                match_started = false;
            }
            _ => {
                match_score_lines.push(line);
            }
        }
    }

    Ok(match_scores)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_log_file_path;

    #[test]
    fn test_read_log_file() {
        let file = File::open(get_log_file_path()).unwrap();
        let result = read_log_file(file);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_game_lines() {
        // grants that the log file is being properly read and
        // the game lines are being correctly identified

        let file = File::open(get_log_file_path()).unwrap();
        let result = read_log_file(file).unwrap();

        assert_eq!(result.scores.len(), 21);
    }
}
