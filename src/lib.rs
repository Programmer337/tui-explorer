use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitStatus;
use std::usize;

use input::*;

mod input;

pub struct Config {
    list_all: bool,
}
impl Config {
    pub fn read_conf() -> Self {
        let as_string: u8 = env::var("LIST_ALL")
            .unwrap_or_else(|_err| "0".to_string())
            .trim()
            .parse()
            .unwrap_or_else(|_err| 0);
        let mut list_all: bool = match as_string {
            0 => false,
            _ => true,
        };

        if env::args().nth(1).unwrap_or("0".to_string()) == "--list-all".to_string() {
            list_all = true;
        };

        Self { list_all }
    }
}

fn open_file(file: &Path) -> io::Result<ExitStatus> {
    print!("Programm zum Öffnen der Datei: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input Error");

    Command::new(input.trim()).arg(file).status()
}

fn print_dirs(paths: &Vec<PathBuf>) -> usize {
    println!("0: ..");
    let mut i: usize = 0;
    for path in paths {
        i += 1;
        println!("{}. {}", i, path.file_name().unwrap().to_str().unwrap());
    }
    i
}

fn filter_elements(elements: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = vec![];
    for i in elements {
        if !i.file_name().unwrap().to_str().unwrap().starts_with(".") {
            result.push(i);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), String> {
    let mut list_all_once = false;
    loop {
        let dir = env::current_dir().unwrap();
        let mut dir = dir.as_path();

        let mut paths = dir
            .read_dir()
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        if !config.list_all && !list_all_once {
            paths = filter_elements(paths);
        }
        list_all_once = false;

        println!("\n{}:", dir.to_str().unwrap());
        let paths_num = print_dirs(&paths);

        let input = match Input::get_input() {
            Ok(input) => match input {
                Input::Choose(size) => {
                    if size > paths_num {
                        eprintln!("Keine Gültige Option");
                        continue;
                    }
                    size
                }
                Input::DirName(name) => {
                    let dir = dir.to_str().unwrap().to_string() + "/" + &name.trim();
                    if Path::new(&dir).is_dir() {
                        if let Err(err) = env::set_current_dir(dir) {
                            if err.kind() == io::ErrorKind::NotFound {
                                eprintln!("Ordner exestiert nicht");
                            } else {
                                eprintln!("Fehler: {}", err.kind());
                            }
                        }
                    }
                    else {
                        println!("{}", if let Err(err) = open_file(Path::new(&name)){
                            if err.kind() == io::ErrorKind::NotFound{
                                "Programm exestiert nicht".to_string()
                            }
                            else {
                                err.kind().to_string()
                            }
                        }else{String::from("")});
                    }
                    continue;
                }
                Input::NewDir(name) => {
                    fs::create_dir(name).unwrap_or_else(|err| {
                        println!("{}", err.kind());
                    });
                    continue;
                }
                Input::Rm(name) => {
                    let as_path = Path::new(&name);
                    if as_path.is_dir() {
                        fs::remove_dir_all(name).unwrap_or_else(|err| {
                            println!("{}", err.kind());
                        });
                    } else {
                        fs::remove_file(as_path).unwrap_or_else(|err| {
                            println!("{}", err.kind());
                        });
                    }
                    continue;
                }
                Input::Command(cmd) => {
                    Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .status().expect("Fehler beim ausführen des Befehls: ");
                    continue;
                }
                Input::ListAll => {
                    list_all_once = true;
                    continue;
                }
                Input::Quit => return Ok(()),
            },
            Err(string) => {
                println!("{string}");
                continue;
            }
        };

        dir = if input == 0 {
            if let Some(path) = dir.parent() {
                path
            } else {
                eprintln!("Kein Überverzeichniss");
                continue;
            }
        } else {
            Path::new(&paths[input - 1])
        };
        if dir.is_file() {
            println!("{}", if let Err(err) = open_file(&dir){
                if err.kind() == io::ErrorKind::NotFound{
                    "Programm exestiert nicht".to_string()
                }
                else {
                    err.kind().to_string()
                }
            }else{String::from("")});
            continue;
        }
        env::set_current_dir(dir).expect("Dieser Ordner konnte nicht geöffnet werden");
    }
}
