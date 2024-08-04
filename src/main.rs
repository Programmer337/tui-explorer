use std::process;

fn main() {
    if let Err(error) = tui_explorer::run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/*Ordner/Dateien löschen
* Ordnernamen eingeben = in diesen navigieren
* Configfunnktion (siehe Turorial)
*/