use std::{ collections::HashMap, io };

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

fn add_task<T: Task>(
    tasks: &mut HashMap<i32, (T, TaskStatus)>,
    description: T,
    next_task_id: &mut i32
) {
    tasks.insert(*next_task_id, (description, TaskStatus::Incomplete));
    *next_task_id += 1;
    println!("Tâche ajoutée !");
}

fn list_tasks<T: Task>(tasks: &HashMap<i32, (T, TaskStatus)>) {
    println!("Liste des tâches :");
    for (id, (description, status)) in tasks {
        println!("{}: {}, Status: {:?}", id, description.description(), status);
    }
}

fn complete_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, id: i32) {
    if let Some((_, status)) = tasks.get_mut(&id) {
        *status = TaskStatus::Complete;
        println!("Tâche marquée comme complète !")
    } else {
        println!("ID de tache invalide.");
    }
}

fn remove_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, id: i32) {
    if tasks.remove(&id).is_some() {
        println!("Tâche supprimée.")
    } else {
        println!("L'ID est invalide !")
    }
}

fn main() {
    let mut tasks: HashMap<i32, (String, TaskStatus)> = HashMap::new();
    let mut next_task_id = 1;

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
            "1" => {
                println!("Entrez une description de la tâche.");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Impossible de lire la ligne.");
                add_task(&mut tasks, description.trim().to_string(), &mut next_task_id);
            }
            "2" => {
                list_tasks(&tasks);
            }
            "3" => {
                println!("Entrez l'ID de la tâche à marquer comme complète.");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Impossible de lire la ligne.");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    complete_task(&mut tasks, id);
                } else {
                    println!("Veuillez entrer un ID valide !");
                }
            }
            "4" => {
                println!("Entrez l'ID de la tâche à supprimer.");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Impossible de lire la ligne.");
                if let Ok(id) = id_str.trim().parse::<i32>() {
                    remove_task(&mut tasks, id);
                } else {
                    println!("Veuillez entrer un ID valide !");
                }
            }
            "5" => {
                println!("Gestionnaire fermé.");
                break;
            }
            _ => println!("Choix incorrect."),
        }
    }
}
