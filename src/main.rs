use calculator::{Calculator, Operator};

mod calculator;

fn read_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("An error occured during the line reading.");
    user_input
}

fn main() {
    println!("Welcome to my basic calculator.");

    loop {
        println!("\nEnter the first number");
        let first_number = match read_user_input().trim().parse::<f64>() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a number only thx.");
                break;
            }
        };

        println!("\nEnter the operator (+,*,-,/)");
        let operator = match read_user_input().trim() {
            "+" => Operator::Add,
            "-" => Operator::Substract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => {
                println!("Enter a correct operator");
                break;
            }
        };

        println!("\nEnter the second number");
        let second_number = match read_user_input().trim().parse::<f64>() {
            Ok(number) => number,
            Err(_) => {
                println!("Enter a number only thx.");
                break;
            }
        };

        let calculator = Calculator::new(first_number, second_number, operator);

        match calculator.calculate() {
            Ok(result) => println!(
                "\n{} {} {} = {}",
                calculator.get_first_number(),
                calculator.get_operator(),
                calculator.get_second_number(),
                result
            ),
            Err(error) => println!("Erreur: {}", error),
        }

        println!("\nDo you still want to continue ? (y/n)");
        if read_user_input().trim().to_lowercase() == "n" {
            println!("See you later.");
            break;
        }
    }
}
