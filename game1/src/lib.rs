use rand::{ thread_rng, Rng };
use ui::clear_terminal;
use utils::functions::read_user_input;

mod utils;

pub fn play_game() -> i64 {
    clear_terminal();

    let words = vec!["rust", "cargo", "developpement", "cli", "training", "game"];
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(0..words.len());
    let target_word = words.get(secret_number).unwrap();
    let target_chars: Vec<char> = target_word.chars().collect();

    let mut correct_guesses: Vec<char> = vec![' '; target_word.len()];
    let mut misplaced_guesses: Vec<char>;
    let mut attempts = 10;
    let score: i64;

    println!("Bienvenue dans le jeu Motus en Rust CLI");
    println!("Vous avez au total {} essais", attempts);

    loop {
        println!("Devinez le mot ({} lettres) !", target_word.len());
        let user_input = read_user_input();

        if !user_input.len().eq(&target_word.len()) {
            if attempts > 0 {
                attempts -= 1;
            }
            println!("Erreur de longueur de mot. Il vous reste {} essai(s)", attempts);
        }

        let user_chars: Vec<char> = user_input.chars().collect();
        misplaced_guesses = Vec::new();
        user_chars
            .iter()
            .enumerate()
            .for_each(|(index, &user_char)| {
                if target_chars.get(index).unwrap().eq(&user_char) {
                    correct_guesses[index] = user_char;
                } else {
                    correct_guesses[index] = ' ';
                    if target_chars.contains(&user_char) {
                        misplaced_guesses.push(user_char);
                    }
                }
            });

        if correct_guesses.iter().all(|&c| c != ' ') {
            println!("Félicitations, vous avez deviné le mot !");
            score = attempts * 10;
            break score;
        }

        println!("Lettres correctes et bien placées : {:?}", correct_guesses);
        println!("Lettres correctes mais mal placées : {:?}", misplaced_guesses);
        if attempts > 0 {
            attempts -= 1;
        }
        println!("Nombre d'essai(s) restant(s) : {}", attempts);
    }
}
