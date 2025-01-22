pub fn clear_terminal() {
    let term = console::Term::stdout();
    term.clear_screen().expect("Une erreur est survenue lors du nettoyage du terminal.");
}
