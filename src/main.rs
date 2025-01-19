use std::io;
use rand::Rng;

fn main() {
    let numbers = [
        rand::thread_rng().gen_range(0..=100),
        rand::thread_rng().gen_range(0..=100),
        rand::thread_rng().gen_range(0..=100),
        rand::thread_rng().gen_range(0..=100),
        rand::thread_rng().gen_range(0..=100),
        rand::thread_rng().gen_range(0..=100),
    ];

    println!("===========================================================");
    println!("👋 Bienvenue dans << Number Hunt> !");
    println!("📜 Objectif : Trouve l'un des nombre cachés.");
    println!("❔ Un tableau de nombres aléatoires a été généré.");
    println!("💭 Devinez un nombre pour voir s'il est bien dans le tableau !");
    println!("===========================================================");

    loop {
        println!("⌨️  Saisis un nombre !");
        match read_and_parse_user_input() {
            Ok(v) => {
                if numbers.iter().any(|x| x == &v) {
                    println!("😁 Bien joué, la valeur {} est bien présente dans le tableau.", v);
                    continue;
                }
                println!("😫 Raté ! Le nombre que tu as saisis n'est pas dans le tableau.");
            }
            Err(_) => println!("😡 Merci de bien vouloir rentrer un nombre."),
        }
    }
}

fn read_and_parse_user_input() -> Result<i32, ()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Impossible de lire la ligne 😵‍💫.");
    match input.trim().parse::<i32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(()),
    }
}
