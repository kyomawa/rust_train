use std::io;

#[derive(Debug)]
enum TaskStatus {
    Incomplete,
    Complete,
}

trait Task {
    fn description(&self) -> String;
    fn status(&self) -> TaskStatus;
}

impl Task for String {
    fn description(&self) -> String {
        self.clone()
    }
    fn status(&self) -> TaskStatus {
        TaskStatus::Incomplete
    }
}

fn main() {
    loop {
        println!("Gestionnaire de liste de tâches");
        println!("1. Ajouter une tâche");
        println!("2. Afficher la liste des tâches");
        println!("3. Marquer une tâche comme complète");
        println!("4. Supprimer une tâche");
        println!("5. Quitter");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Impossible de lire la ligne.");

        match choice.trim() {
            "5" => {
                println!("Gestionnaire fermé.");
                break;
            }
            _ => println!("Choix incorrect."),
        }
    }
}
