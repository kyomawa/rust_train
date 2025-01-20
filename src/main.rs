use bkit::read_user_input;

fn main() {
    println!("");
    println!("================================================================");
    println!("");
    println!("Veuillez écrire quelque chose !");
    println!("");
    println!("================================================================");
    println!("");

    loop {
        let user_input = read_user_input();

        println!("");
        println!("================================================================");
        println!("");
        println!("Vous avez écris : {}", user_input);
        println!("");
        println!("================================================================");
        println!("");
        println!("Veuillez écrire quelque chose d'autre !");
        println!("");
        println!("================================================================");
        println!("");
    }
}
