pub fn read_user_input() -> String {
    let mut user_input = String::new();
    std::io
        ::stdin()
        .read_line(&mut user_input)
        .expect("Une erreur est survenue lors de la lecture de la ligne.");
    user_input = user_input.trim().to_string();
    user_input
}
