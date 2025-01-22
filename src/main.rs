use database::sqlite::initiate_db;
use models::{ game::GAMES, user::get_user };
use models::score::show_all_scores;
use utils::functions::read_user_input;
use rusqlite::{ Connection, Result };
use models::game::play_game;

mod models;
mod database;
mod utils;

fn main() -> Result<()> {
    let conn = Connection::open("scores.sqlite").expect("Erreur lors de l'accès à la BDD.");
    initiate_db(&conn);

    println!("================================================================");
    println!("\n👋 Bienvenue dans le HUB.");
    println!("⌨️   Renseigne un pseudo pour commencer.");
    println!("\n================================================================\n");

    let username = read_user_input();

    let user = get_user(&conn, &username);

    println!("\n😁 Amuse toi bien {}.\n", username);

    loop {
        println!("================================================================\n");
        println!("⌨️   Saisis un chiffre pour effectuer ton choix.");
        println!("\n🎮  1. Jouer au Motus.");
        println!("🎮  2. Jouer à ReactGuy.");
        println!("🎮  3. Jouer au Guessing game.");
        println!("📺  4. Afficher les scores.");
        println!("🐾  5. Quitter.");
        println!("\n================================================================\n");

        let choice: String = read_user_input();
        let choice = match choice.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("\n🥸  Veuillez renseigner un chiffre.\n");
                continue;
            }
        };

        match choice {
            1 =>
                match play_game(&conn, &user, GAMES[0]) {
                    Ok(score) => println!("Joli score de {}", score),
                    Err(e) => println!("{}", e),
                }
            2 => println!("\n🤔 Choix pas encore implémenté !\n"),
            3 => println!("\n🤔 Choix pas encore implémenté !\n"),
            4 => show_all_scores(&conn, &user)?,
            5 => {
                println!("\n🫡  À la prochaine !\n");
                break Ok(());
            }
            _ => println!("\n😡 Choix non valide:\n"),
        }
    }
}
