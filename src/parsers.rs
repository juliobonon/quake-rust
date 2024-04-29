use super::scores::match_score::MatchScore;
use super::scores::player_score::PlayerScores;
use regex::Regex;

pub fn extract_players_and_means(input: &str) -> Option<(&str, &str, &str)> {
    let re = r"\d:\s(.*)\skilled\s(.*)\sby\s(.*)";
    let captures = Regex::new(re).unwrap().captures(&input);

    captures.map(|capture| {
        let killer = capture.get(1).unwrap().as_str();
        let killed = capture.get(2).unwrap().as_str();
        let mean = capture.get(3).unwrap().as_str();

        (killer, killed, mean)
    })
}

pub fn parse_game_lines(game_lines: &Vec<String>) -> MatchScore {
    let mut match_score = MatchScore::new();
    let mut player_scores = PlayerScores::new();

    for line in game_lines {
        if line.contains("Kill") {
            if let Some((killer, killed, mean)) = extract_players_and_means(&line) {
                match_score.total_kills += 1;
                match_score.increase_kill_by_means(mean);
                player_scores.add_kill(killer);
                player_scores.remove_kill(killed);
            }
        }
    }
    match_score.players = player_scores.get_all_players();
    match_score.kills = player_scores.get_players_scores();

    match_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extract_players_and_means() {
        let input = " 22 1: Player1 killed Player2 by MOD_RAILGUN";
        let (killer, killed, mean) = extract_players_and_means(input).unwrap();

        assert_eq!(killer, "Player1");
        assert_eq!(killed, "Player2");
        assert_eq!(mean, "MOD_RAILGUN");
    }

    #[test]
    fn test_parse_game_lines() {
        let game_lines = vec![
            " 10:32 Kill: 7 5 7: Mal killed Assasinu Credi by MOD_ROCKET_SPLASH".to_string(),
            " 10:30 Kill: 3 4 7: Oootsimo killed Dono da Bola by MOD_ROCKET_SPLASH".to_string(),
            " 10:27 Kill: 3 6 7: Oootsimo killed Zeh by MOD_ROCKET_SPLASH".to_string(),
            " 2 4 1 Kill: 3 6 7: Player1 killed Player4 by MOD_RAILGUN".to_string(),
            " 2 3 1 Kill: 3 6 7: Player1 killed Player5 by MOD_RAILGUN".to_string(),
            " 2 4 1 Kill: 3 6 7: Player1 killed Player6 by MOD_RAILGUN".to_string(),
            "20:54 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT".to_string(),
        ];
        let players_score = HashMap::from([
            ("Mal".to_string(), 1),
            ("Assasinu Credi".to_string(), -1),
            ("Oootsimo".to_string(), 2),
            ("Dono da Bola".to_string(), -1),
            ("Zeh".to_string(), -1),
            ("Player1".to_string(), 3),
            ("Player4".to_string(), -1),
            ("Player5".to_string(), -1),
            ("Player6".to_string(), -1),
            ("Isgalamido".to_string(), -1),
        ]);

        let match_score = parse_game_lines(&game_lines);
        println!("{:#?}", match_score);
        assert_eq!(match_score.total_kills, 7);
        assert_eq!(match_score.players.len(), 10);
        assert_eq!(match_score.kills, players_score);
        assert_eq!(
            match_score.kills_by_means,
            HashMap::from([
                ("MOD_ROCKET_SPLASH".to_string(), 3),
                ("MOD_RAILGUN".to_string(), 3),
                ("MOD_TRIGGER_HURT".to_string(), 1),
            ])
        );
    }
}
