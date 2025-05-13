use tip_calculator::TipCalculator;
use utils::read_user_input;

mod tip_calculator;
mod utils;

fn main() {
    println!("💰 Tips Calculator");
    loop {
        println!("✍️  Please enter the amount of the bill.");
        let bill = read_user_input();

        println!("\n✍️  Please enter the tip percent.");
        let tip_percent = read_user_input();

        let tip_calculator = match TipCalculator::new(bill, tip_percent) {
            Ok(tc) => tc,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        println!("\n🤑 The tip: {}", tip_calculator.calculate_tip());
        println!("💲 Total: {}", tip_calculator.calculate_total());
    }
}
