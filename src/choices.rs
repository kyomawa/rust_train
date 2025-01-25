use rand::{ thread_rng, Rng };
use rand::prelude::SliceRandom;

pub fn generate_strong_password() -> String {
    let chars =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
  abcdefghijklmnopqrstuvwxyz\
  0123456789\
  !@#$%^&*()-_=+{}[]|:;,.<>/?";
    let mut rng = thread_rng();
    let password = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();
    password
}

pub fn generate_clean_password() -> String {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let all_chars = format!("{}{}{}", lowercase, uppercase, digits);

    let mut rng = thread_rng();

    let mut generate_group = || {
        let mut group = String::new();

        // Garantir au moins 1 chiffre
        group.push(
            digits
                .chars()
                .nth(rng.gen_range(0..digits.len()))
                .unwrap()
        );

        // Ajouter 4 caractères aléatoires parmi minuscules, majuscules et chiffres
        for _ in 0..4 {
            let idx = rng.gen_range(0..all_chars.len());
            group.push(all_chars.chars().nth(idx).unwrap());
        }

        // Ajouter 1 majuscule
        group.push(
            uppercase
                .chars()
                .nth(rng.gen_range(0..uppercase.len()))
                .unwrap()
        );

        // Mélanger les caractères du groupe
        let mut chars: Vec<char> = group.chars().collect();
        chars.shuffle(&mut rng);

        // Convertir en String
        chars.into_iter().collect::<String>()
    };

    // Générer 3 groupes séparés par des tirets
    let password = (0..3)
        .map(|_| generate_group())
        .collect::<Vec<_>>()
        .join("-");

    password
}
