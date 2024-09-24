use std::collections::HashMap;

// Representerer poenget en deltager har fått i en lek
#[derive(Debug, Clone)]
pub struct PlayerScore {
    pub player_name: String,
    pub score: u32,
}

// Representerer et spill med et sett deltagere og deres poeng i det spillet
#[derive(Debug, Clone)]
pub struct Game {
    pub game_name: String,
    pub scores: Vec<PlayerScore>,
}

// Representerer et sett med leiker
#[derive(Debug)]
pub struct GamesOnDate {
    pub date: String,
    pub games: Vec<Game>,
}

impl GamesOnDate {
    // Beregner produktet en deltager har for denne datoen
    pub fn total_score_for_player(&self, player_name: &str) -> u32 {
        self.games.iter().fold(1, |acc, game| { // itererer over alle spill, sender inn en closure
                                                // hvor første argument er en variabel som holder
                                                // poengsummen og andre er spillet
            let score = game
                .scores
                .iter()
                .find(|ps| ps.player_name == player_name) // `find` tar inn en conditional closure
                                                          // og returnerer første element som
                                                          // passer conditionalen
                .map(|ps| ps.score) // konverterer fra `PlayerScore` til en `score`
                .unwrap_or(1); // hvis deltageren ikke har en score så blir det satt til 1

            if score == 0 {
                acc // hvis score er 0 så ignorerer vi den
            } else {
                acc * score
            }
        })
    }

    // Genererer en tabell over alle spill og deltagere med deres poeng. Deltagere som rader og spill
    // som kolonner, der siste kolonne er totalen per deltager
    pub fn to_table(&self) -> Vec<Vec<String>> {
        let mut table = Vec::new();
        
        // Lager headerne med alle spillnavnene og total-kolonnen
        let mut header = vec!["Deltager".to_string()]; // første kolonne som er deltagere
        for game in &self.games {
            header.push(game.game_name.clone()); // legger til alle leikene
        }
        header.push("Total".to_string()); // legger til total-kolonnen
        table.push(header);
    
        // Henter ut alle deltagerene som deltaer
        let mut player_names = self.games.iter() // iterer over alle leika
            // henter ut alle deltagernavn og flater det ut til en flat iterator
            .flat_map(|game| game.scores.iter().map(|score| score.player_name.clone()))
            .collect::<Vec<_>>(); // konverterer iteratoren til en vektor
        
        player_names.sort(); // sorterer alfabetisk
        player_names.dedup(); // fjerner eventuelle duplikater av navn
    
        // Lager vektor for radene for hver deltager
        let mut player_rows = Vec::new();
    
        // iterere over alle de unike deltagernavnene 
        for player_name in player_names {
            let mut row = vec![player_name.clone()]; // første kolonne er deltagernavnet
            let mut total_score = 1; // beregner totalen, går raskere å gjøre det her enn å kalle
                                     // funksjonen over
            let mut participated = false; 
    
            for game in &self.games {
                let score = game.scores.iter()
                    .find(|ps| ps.player_name == player_name)
                    .map(|ps| ps.score)
                    .unwrap_or(0); // henter ut poenget til den spesifikke spilleren, 0 hvis spiller ikke er nevnt
                
                row.push(score.to_string());
                
                if score != 0 {
                    total_score *= score;
                    participated = true;
                }
            }
    
            // hvis deltageren ikke har vært med i noen spill så totalen satt til "N/A"
            if !participated {
                row.push("N/A".to_string());
            } else {
                row.push(total_score.to_string());
            }
    
            player_rows.push((total_score, row));
        }
    
        // sorterer etter radene etter totalen
        player_rows.sort_by(|a, b| {
        match (a.1.last().unwrap().as_str(), b.1.last().unwrap().as_str()) {
            ("N/A", "N/A") => std::cmp::Ordering::Equal,
            ("N/A", _) => std::cmp::Ordering::Greater,
            (_, "N/A") => std::cmp::Ordering::Less,
            (a_total, b_total) => a_total.parse::<i32>().unwrap().cmp(&b_total.parse::<i32>().unwrap()),
        }
        });
    
        // Add sorted rows to the table
        for (_, row) in player_rows {
            table.push(row);
        }
    
        table
    }

}

// Representerer kartet over alle datoer med leiker i
#[derive(Debug)]
pub struct GameData {
    pub data: HashMap<String, GamesOnDate>,
}

impl GameData {
    // Legger til et spill på en spesifikk dato
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


    // Henter ut tabellen for en gitt dato
    pub fn get_table_for_date(&self, date: &str) -> Option<Vec<Vec<String>>> {
        self.data.get(date).map(|games_on_date| games_on_date.to_table())
    }
}

