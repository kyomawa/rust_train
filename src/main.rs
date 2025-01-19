use std::{ cmp::Ordering, collections::HashMap, io };

use rand::{ thread_rng, Rng };

fn instructions() {
    println!("");
    println!("==========================================================");
    println!("");
    println!("Bienvenue dans le guessing game v2.");
    println!("Un nombre aléatoire a été généré.");
    println!("Objectif: Trouver le nombre en 10 essais.");
    println!("");
    println!("==========================================================");
    println!("");
}

fn read_user_input_and_parse() -> Option<u8> {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Une erreur est survenue lors de la lecture de la ligne.");
    match user_input.trim().parse::<u8>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Une erreur est survenue lors de la lecture de la ligne.");
    user_input
}

fn main() {
    let secret_number: u8 = thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
    let mut score: HashMap<&str, u16> = HashMap::new();
    let mut attempts: u8 = 10;

    instructions();

    loop {
        println!("Veuillez rentrer un nombre.");
        match read_user_input_and_parse() {
            Some(user_number) => {
                match secret_number.cmp(&user_number) {
                    Ordering::Greater => {
                        println!("Le nombre caché est supérieur !");
                        if attempts != 0 {
                            attempts -= 1;
                        }
                    }
                    Ordering::Less => {
                        println!("Le nombre caché est inférieur !");
                        if attempts != 0 {
                            attempts -= 1;
                        }
                    }
                    Ordering::Equal => {
                        let user_score = attempts * 10;
                        println!("Bingo tu l'as trouvé batard !");
                        println!("Ton score est de {} points", user_score);
                        println!("Quel est ton nom ?");
                        let user_name = read_user_input();
                        score.insert(&user_name, user_score.into());
                        break;
                    }
                }
            }
            None => println!("Veuillez rentrer un nombre par pitié."),
        }
    }
}
