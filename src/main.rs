use utils::functions::read_user_input;

mod models;
mod database;
mod utils;

fn main() {
    println!("================================================================");
    println!("\n👋 Bienvenue dans le HUB.");
    println!("⌨️   Renseigne un pseudo pour commencer.");
    println!("\n================================================================\n");

    let username = read_user_input();

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
            1 => println!("\n🤔 Choix pas encore implémenté !\n"),
            2 => println!("\n🤔 Choix pas encore implémenté !\n"),
            3 => println!("\n🤔 Choix pas encore implémenté !\n"),
            4 => println!("\n🤔 Choix pas encore implémenté !\n"),
            5 => {
                println!("\n🫡  À la prochaine !\n");
                break;
            }
            _ => println!("\n😡 Choix non valide:\n"),
        }
    }
}
