fn main() {
    println!("Voici les tables de multiplication :");

    let mut multiple = 1;

    while multiple <= 10 {
        println!("\nTable du {} :", multiple);

        let mut number = 1;

        while number <= 10 {
            println!("  {} x {} = {}", multiple, number, multiple * number);
            number += 1;
        }

        multiple += 1;
    }
}
