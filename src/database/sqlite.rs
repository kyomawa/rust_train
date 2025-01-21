use rusqlite::Connection;

use crate::models::game::{ add_game, GAMES };

pub fn initiate_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users ( id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE)",
        ()
    ).expect("Une erreur lors de la création de la table users est arrivé.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games ( id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE)",
        ()
    ).expect("Une erreur lors de la création de la table games est arrivé.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS scores ( id INTEGER PRIMARY KEY, score INTEGER NOT NULL, user_id INTEGER NOT NULL, game_id INTEGER NOT NULL, FOREIGN KEY (user_id) REFERENCES users(id), FOREIGN KEY (game_id) REFERENCES games(id))",
        ()
    ).expect("Une erreur lors de la création de la table scores est arrivé.");

    GAMES.iter().for_each(|game| {
        add_game(conn, game);
    });
}
