use std::process;

fn main() {
    if let Err(error) = tui_explorer::run() {
        eprintln!("{error}");
        process::exit(1);
    }
}

/*UI verbessern 
* Ordner erstellen funktion
* Configfunnktion (siehe Turoria)
*/