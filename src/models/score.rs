use rusqlite::{ Connection, Result };
use crate::models::user::User;

struct Score {
    id: i64,
}

pub fn add_score(conn: &Connection, user_id: i64, game_id: i64, score: i64) {
    conn.execute(
        "INSERT INTO scores (user_id, game_id, score) VALUES (?1, ?2, ?3)",
        &[&user_id, &game_id, &score]
    ).expect("Une erreur est survenue lors de l'ajout du score.");
}

pub fn show_all_scores(conn: &Connection, user: &User) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT games.name, scores.score FROM scores INNER JOIN users ON users.id = scores.user_id INNER JOIN games ON games.id = scores.game_id WHERE users.id = ?1"
    )?;

    println!("\nScores pour l'utilisateur {} :", user.name);
    let mut rows = stmt.query(rusqlite::params![user.id])?;
    while let Some(row) = rows.next()? {
        let game: String = row.get(0)?;
        let score: i32 = row.get(1)?;
        println!("🕹️  {} | 🥅 Score: {}", game, score);
    }
    println!("");
    Ok(())
}
