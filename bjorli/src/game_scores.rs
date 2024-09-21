use crate::game_data_structure::GameData;


pub fn add_2021_09(game_data: &mut GameData) {

    // Define the date in question
    let date = "2021-09";

    // Add each game with participant scores
    game_data.add_game(
        date,
        "Kjærleiksbrev",
        vec![
            ("Joakim".to_string(), 2),
            ("Bendik".to_string(), 1),
            ("Peter".to_string(), 7),
            ("Roar".to_string(), 5),
            ("Morten".to_string(), 3),
            ("Are".to_string(), 4),
            ("Trond".to_string(), 6),
        ],
    );

    game_data.add_game(
        date,
        "Ølmesternes mester",
        vec![
            ("Joakim".to_string(), 3),
            ("Bendik".to_string(), 6),
            ("Peter".to_string(), 2),
            ("Roar".to_string(), 5),
            ("Morten".to_string(), 4),
            ("Are".to_string(), 1),
            ("Trond".to_string(), 7),
        ],
    );

    game_data.add_game(
        date,
        "The Amazing Labyrinth",
        vec![
            ("Joakim".to_string(), 2),
            ("Bendik".to_string(), 4),
            ("Peter".to_string(), 6),
            ("Roar".to_string(), 3),
            ("Morten".to_string(), 5),
            ("Are".to_string(), 7),
            ("Trond".to_string(), 1),
        ],
    );

    game_data.add_game(
        date,
        "Darts",
        vec![
            ("Joakim".to_string(), 4),
            ("Bendik".to_string(), 5),
            ("Peter".to_string(), 2),
            ("Roar".to_string(), 1),
            ("Morten".to_string(), 3),
            ("Are".to_string(), 6),
            ("Trond".to_string(), 7),
        ],
    );

    game_data.add_game(
        date,
        "7 shots of glory",
        vec![
            ("Joakim".to_string(), 3),
            ("Bendik".to_string(), 6),
            ("Peter".to_string(), 5),
            ("Roar".to_string(), 2),
            ("Morten".to_string(), 1),
            ("Are".to_string(), 4),
            ("Trond".to_string(), 7),
        ],
    );

    game_data.add_game(
        date,
        "Blindtest",
        vec![
            ("Joakim".to_string(), 2),
            ("Bendik".to_string(), 1),
            ("Peter".to_string(), 3),
            ("Roar".to_string(), 6),
            ("Morten".to_string(), 5),
            ("Are".to_string(), 4),
            ("Trond".to_string(), 7),
        ],
    );

    game_data.add_game(
        date,
        "NattBoccia",
        vec![
            ("Joakim".to_string(), 5),
            ("Bendik".to_string(), 3),
            ("Peter".to_string(), 2),
            ("Roar".to_string(), 6),
            ("Morten".to_string(), 7),
            ("Are".to_string(), 4),
            ("Trond".to_string(), 1),
        ],
    );
}
