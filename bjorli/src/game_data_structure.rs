use std::collections::HashMap;

// Represents the score of a player for a game
#[derive(Debug, Clone)]
pub struct PlayerScore {
    pub player_name: String,
    pub score: u32,
}

// Represents a single game on a particular date
#[derive(Debug, Clone)]
pub struct Game {
    pub game_name: String,
    pub scores: Vec<PlayerScore>,
}

// Represents all games for a single date
#[derive(Debug)]
pub struct GamesOnDate {
    pub date: String,
    pub games: Vec<Game>,
}

impl GamesOnDate {
    // Calculate the total score product for a player on this date, excluding 0s
    pub fn total_score_for_player(&self, player_name: &str) -> u32 {
        self.games.iter().fold(1, |acc, game| {
            let score = game
                .scores
                .iter()
                .find(|ps| ps.player_name == player_name)
                .map(|ps| ps.score)
                .unwrap_or(1); // Use 1 if player didn't play this game

            if score == 0 {
                acc // skip multiplying by 0
            } else {
                acc * score
            }
        })
    }

    // Generate data in a tabular form where the first column is player names, each subsequent column is their score per game, and the last column is the total score
    pub fn to_table(&self) -> Vec<Vec<String>> {
        let mut table = Vec::new();
        
        // Create the header row with game names and an additional "Total" column
        let mut header = vec!["Player".to_string()];
        for game in &self.games {
            header.push(game.game_name.clone());
        }
        header.push("Total".to_string());
        table.push(header);

        // Collect a unique list of player names from all games
        let mut player_names = self.games.iter()
            .flat_map(|game| game.scores.iter().map(|score| score.player_name.clone()))
            .collect::<Vec<_>>();
        
        player_names.sort();
        player_names.dedup();

        // Create rows for each player with their scores
        for player_name in player_names {
            let mut row = vec![player_name.clone()];
            let mut total_score = 1;
            let mut participated = false;

            for game in &self.games {
                let score = game.scores.iter()
                    .find(|ps| ps.player_name == player_name)
                    .map(|ps| ps.score)
                    .unwrap_or(0); // 0 if player didn't play this game
                
                row.push(score.to_string());
                
                if score != 0 {
                    total_score *= score;
                    participated = true;
                }
            }

            // If the player didn't participate in any game, set total_score to "N/A"
            if !participated {
                row.push("N/A".to_string());
            } else {
                row.push(total_score.to_string());
            }

            table.push(row);
        }

        table
    }
}

// Represents the overall data structure containing data across multiple dates
#[derive(Debug)]
pub struct GameData {
    pub data: HashMap<String, GamesOnDate>,
}

impl GameData {
    // Add a new game on a specific date
    pub fn add_game(&mut self, date: &str, game_name: &str, player_scores: Vec<(String, u32)>) {
        let game = Game {
            game_name: game_name.to_string(),
            scores: player_scores
                .into_iter()
                .map(|(player_name, score)| PlayerScore { player_name, score })
                .collect(),
        };

        self.data
            .entry(date.to_string())
            .or_insert(GamesOnDate {
                date: date.to_string(),
                games: Vec::new(),
            })
            .games
            .push(game);
    }

    // Get total score for a player on a specific date
    pub fn get_total_score(&self, date: &str, player_name: &str) -> Option<u32> {
        self.data.get(date).map(|games_on_date| {
            games_on_date.total_score_for_player(player_name)
        })
    }

    // Get a table representation of the games played on a specific date
    pub fn get_table_for_date(&self, date: &str) -> Option<Vec<Vec<String>>> {
        self.data.get(date).map(|games_on_date| games_on_date.to_table())
    }
}

