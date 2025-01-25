fn main() {
    // Crée une nouvelle instance de Winres
    let mut res = winres::WindowsResource::new();

    // Spécifie le chemin vers ton fichier .ico
    res.set_icon("icon.ico");

    // Compile et intègre l'icône
    res.compile().expect("Erreur lors de la compilation des ressources");
}
