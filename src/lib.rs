use config::Config;

mod config;

pub fn run() {
    loop {
        println!("Please enter a valid command to continue.");
        println!("Valid commands:\n  minigrep search\n  minigrep quit");

        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("An error occured during the line reading");

        match user_input.trim() {
            "minigrep search" => {
                let mut user_answer = String::new();
                println!("What is the path of the file you want so search in ?");
                std::io::stdin()
                    .read_line(&mut user_answer)
                    .expect("An error occured during the line reading");

                let mut user_answer2 = String::new();
                println!(
                    "What is the character, word or sentence you want to search in the file: {} ?",
                    &user_answer
                );
                std::io::stdin()
                    .read_line(&mut user_answer2)
                    .expect("An error occured during the line reading");

                let config = Config::new(
                    user_answer2.trim().to_string(),
                    user_answer.trim().to_string(),
                )
                .unwrap();
                if config.is_file_having_query() {
                    println!(
                        "The file: {} contains the query: {}",
                        config.get_file_path(),
                        config.get_query()
                    )
                } else {
                    println!(
                        "The file: {} do not contains the query: {}",
                        config.get_file_path(),
                        config.get_query()
                    )
                }
            }
            "minigrep quit" => {
                println!("Program exiting...");
                break;
            }
            _ => {
                println!("Please enter a valid command to continue.");
                println!("Valid commands:\n  minigrep search\n  minigrep quit");
            }
        }
    }
}
