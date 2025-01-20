use std::io;

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("An error occured during the line reading.");
    user_input.trim().to_string()
}
