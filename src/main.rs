use std::io;

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Impossible de lire la ligne !");
    String::from(user_input.trim())
}

fn instructions() {
    println!("");
    println!("===========================================================");
    println!("");
    println!("Bienvenue dans sentence analyzer");
    println!("Ce programme analyse une phrase pour donner des informations sur cette dernière.");
    println!("");
    println!("===========================================================");
    println!("");
    println!("Veuillez saisir votre phrase.");
    println!("");
    println!("===========================================================");
    println!("");
}

fn main() {
    let vowels = "aeyuioAEYUIO";

    instructions();

    loop {
        let user_input = read_user_input();

        println!("");

        let sentence_vowels_count = user_input
            .chars()
            .filter(|c| vowels.contains(*c))
            .count();
        println!("Votre phrase contient {} voyelles.", sentence_vowels_count);

        let clean_user_input: String = user_input
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        let is_palindrome = clean_user_input
            .to_lowercase()
            .chars()
            .eq(clean_user_input.to_lowercase().chars().rev());

        if is_palindrome {
            println!("Votre phrase est un palindrome.");
        } else {
            println!("Votre phrase n'est pas un palindrome.");
        }

        let mut words: Vec<&str> = user_input.split_whitespace().collect();
        words.sort_by_key(|&word| word.len());
        println!("Mots triés par longueur : {:?}", words);

        let acronym: String = user_input
            .split_whitespace()
            .map(|word| word.chars().next().unwrap_or(' '))
            .collect();
        println!("L'acronyme de la phrase est : {}", acronym);

        println!("");
        println!("===========================================================");
        println!("");
        println!("Veuillez saisir une nouvelle phrase à analyser.");
        println!("");
        println!("===========================================================");
        println!("");
    }
}
