use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerScores {
    players: Vec<PlayerScore>,
}
impl PlayerScores {
    pub fn new() -> Self {
        PlayerScores {
            players: Vec::new(),
        }
    }

    fn add_player(&mut self, player: PlayerScore) {
        self.players.push(player);
    }

    pub fn add_kill(&mut self, player: &str) {
        match self.get_player(player) {
            Some(p) => p.kills += 1,
            None => {
                let player = PlayerScore::new(player.to_string(), 1);
                self.add_player(player)
            }
        }
    }

    pub fn remove_kill(&mut self, player: &str) {
        match self.get_player(player) {
            Some(p) => p.kills -= 1,
            None => {
                let player = PlayerScore::new(player.to_string(), -1);
                self.add_player(player);
            }
        }
    }

    fn get_player(&mut self, name: &str) -> Option<&mut PlayerScore> {
        self.players.iter_mut().find(|p| p.name == name)
    }

    pub fn get_all_players(&self) -> Vec<String> {
        // get all players in the game except <world>
        self.players
            .iter()
            .filter(|p| p.name != "<world>")
            .map(|p| p.name.clone())
            .collect()
    }

    pub fn get_players_scores(&self) -> HashMap<String, i32> {
        // get all players scores from the game except <world>
        self.players
            .iter()
            .filter(|p| p.name != "<world>")
            .map(|p| (p.name.clone(), p.kills))
            .collect()
    }
}

#[derive(Debug)]
struct PlayerScore {
    name: String,
    kills: i32,
}

impl PlayerScore {
    fn new(name: String, kills: i32) -> Self {
        PlayerScore { name, kills }
    }
}
