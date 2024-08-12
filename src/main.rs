use std::process;

fn main() {
    if let Err(error) = tui_explorer::run(tui_explorer::Config::read_conf()){
        eprintln!("{error}");
        process::exit(1);
    }
}