use std::cmp::Ordering;
use std::io;
use rand::{ thread_rng, Rng };

fn main() {
    let mut secret: i32 = thread_rng().gen_range(1..=100);

    println!("Entre un nombre entre 1 et 100 !");

    loop {
        let mut input = String::new();

        match read_and_compare(&mut input, &secret) {
            Some(Ordering::Equal) => {
                println!("Bingo, t'as trouvé");
                secret = thread_rng().gen_range(1..=100);
                println!("Un nouveau nombre aléatoire entre 1 et 100 a été généré !");
                println!("Entre un nombre entre 1 et 100 !");
                continue;
            }
            Some(Ordering::Greater) => println!("Le nombre à trouver est inférieur !"),
            Some(Ordering::Less) => println!("Le nombre à trouver est supérieur !"),
            None => {
                continue;
            }
        }
    }
}

fn read_and_compare(input: &mut String, secret: &i32) -> Option<Ordering> {
    match io::stdin().read_line(input) {
        Ok(_) => {
            let guess: i32 = match input.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Veuillez renseigner un nombre valide !");
                    return None;
                }
            };

            Some(guess.cmp(secret))
        }
        Err(_) => {
            println!("Impossible de lire la ligne !");
            return None;
        }
    }
}
