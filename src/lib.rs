use std::env;
use std::io::{self, Write};
use std::fs;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::usize;

enum Input{
    Choose(usize),
    DirName(String),
    NewDir(String),
    Rm(String),
    Quit,
}

fn get_input() -> Result<Input, String>{
    print!("Wähle eine Option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    match input.trim(){
        "q" => Ok(Input::Quit),
        "mkdir" => {
            input = String::from("");
            print!("Name: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Input error");
            Ok(Input::NewDir(input.trim().to_string()))
        },
        "rm" => {
            input = String::from("");
            print!("Name: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Input error");

            Ok(Input::Rm(input.trim().to_string()))
        }
        _ => {
            if let Ok(usize) = input.trim().parse(){
                Ok(Input::Choose(usize))
            }
            else{
                Ok(Input::DirName(input))
            }
        },
    }
}

fn open_file(file: &Path) -> io::Error{
    print!("Programm zum Öffnen der Datei: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    return Command::new(input.trim())
        .arg(file)
        .exec();
}

fn print_dirs (paths: fs::ReadDir) -> usize {
    println!("0: ..");
        let mut i: usize = 0;
        for path in paths{
            i += 1;
            println!("{}. {}", i, path.unwrap().path().file_name().unwrap().to_str().unwrap());
        }
        i
}

pub fn run() -> Result<(), String>{
    loop{
        let dir = env::current_dir().unwrap();
        let mut dir = dir.as_path();

        let paths = dir.read_dir().unwrap();

        println!("\n{}:", dir.to_str().unwrap());
        let paths_num = print_dirs(paths);

        let input = match get_input() {
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