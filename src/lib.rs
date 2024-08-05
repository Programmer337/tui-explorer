use std::env;
use std::io::{self, Write};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::usize;

use input::*;

mod input;

pub struct Config{
    list_all: bool,
}
impl Config {
    pub fn read_conf() -> Self{
        let as_string: u8 = env::var("list-all").unwrap_or_else(|_err|{
            "0".to_string()
        }).trim().parse().unwrap_or_else(|_err|{
            0
        });
        let mut list_all: bool = match as_string {
            0 => false,
            _ => true,
        };

        if env::args().nth(1).unwrap_or("0".to_string()) == "--list-all".to_string(){
            list_all = true;
        };

        Self{list_all}
    }
}

fn open_file(file: &Path) -> io::Error{
    print!("Programm zum Öffnen der Datei: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    Command::new(input.trim())
        .arg(file)
        .exec()
}

fn print_dirs (paths: fs::ReadDir, list_all: bool) -> usize {
    println!("0: ..");
    let mut i: usize = 0;
    for path in paths{
        let dir_entry = &path.unwrap();
        let path = &dir_entry.path();
        let element = path.file_name().unwrap();
        if list_all || !element.to_str().unwrap().starts_with("."){
            i += 1;
            println!("{}. {}", i, element.to_str().unwrap());
        }
    }
    i
}
pub fn run(config: Config) -> Result<(), String>{
    loop{
        let dir = env::current_dir().unwrap();
        let mut dir = dir.as_path();

        let paths = dir.read_dir().unwrap();

        println!("\n{}:", dir.to_str().unwrap());
        let paths_num = print_dirs(paths, config.list_all);

        let input = match Input::get_input() {
            Ok(input) => match input{
                    Input::Choose(size) => {
                        if size > paths_num {
                            eprintln!("Keine Gültige Option");
                            continue;
                        }
                        size
                    },
                    Input::NewDir(name) => {
                        fs::create_dir(name).unwrap_or_else(|err|{
                            println!("{}", err.kind());
                        });
                        continue;
                    }
                    Input::Rm(name) => {
                        let as_path = Path::new(&name);
                        if as_path.is_dir(){
                            fs::remove_dir_all(name).unwrap_or_else(|err|{
                                println!("{}", err.kind());
                            });
                        }
                        else {
                            fs::remove_file(as_path).unwrap_or_else(|err|{
                                println!("{}", err.kind());
                            });
                        }
                        continue;
                    }
                    Input::Quit => return Ok(()),
                    _ => {
                        println!("Bitte gib eine gültige Option ein");
                        continue;
                    }
                },
            Err(string) => {
                println!("{string}"); 
                continue;
            },
        };

        let mut paths = dir.read_dir().unwrap();
        let as_path: PathBuf;
        dir = if input == 0{
            if let Some(path) = dir.parent(){
                path
            }
            else {
                eprintln!("Kein Überverzeichniss");
                continue;
            }
        }
        else {
            as_path = paths.nth(input-1).unwrap().unwrap().path();
            Path::new(&as_path)
        };
        if dir.is_file(){
            println!("{}", open_file(&dir));
            continue;
        }
        env::set_current_dir(dir).expect("Dieser Ordner konnte nicht geöffnet werden");
    } 
}