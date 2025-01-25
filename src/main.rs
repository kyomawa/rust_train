mod utils;
mod choices;

use choices::{ generate_strong_password, generate_clean_password };
use utils::read_user_input;

fn main() {
    println!("\n==========================================================================\n");
    println!("👋 Bienvenue sur le générateur de mot de passe aléatoire !");

    loop {
        println!("\n==========================================================================\n");
        println!("🎲 1. Générer un mot de passe propre");
        println!("🎲 2. Générer un mot de passe robuste");
        println!("\n==========================================================================\n");

        let choice = read_user_input();
        let choice: Result<u8, ()> = match choice.parse::<u8>() {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        };

        match choice {
            Ok(number) => {
                match number {
                    1 => {
                        let password = generate_clean_password();
                        println!("\n🔑 Mot de passe: {} \n", password);
                    }
                    2 => {
                        let password = generate_strong_password();
                        println!("\n🔑 Mot de passe: {} \n", password);
                    }
                    _ => println!("❌ Choix incorrect, fais un effort..."),
                }
            }
            Err(_) => println!("❌ Merci de bien vouloir renseigner un chiffre !"),
        }
    }
}
