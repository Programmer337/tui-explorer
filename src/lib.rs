use std::env;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;


fn get_input() -> Result<usize, String>{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    if input.trim() == "q"{
        return Err("quit".to_string());
    }
    let input: usize = if let Ok(usize) = input.trim().parse(){
        usize
    }
    else{
        return Err("Eingabe ist keine Zahl".to_string());
    };
    Ok(input)
}

fn open_file(file: &Path) -> io::Error{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    return Command::new(input.trim())
        .arg(file)
        .exec();
}

pub fn run() -> Result<(), String>{
    loop{
        let dir = env::current_dir().unwrap();
        let mut dir = dir.as_path();

        let paths = dir.read_dir().unwrap();
        let paths_num = dir.read_dir().unwrap().count();

        println!("0: ..");
        let mut i: u32 = 0;
        for path in paths{
            i += 1;
            println!("{}. {}", i, path.unwrap().path().file_name().unwrap().to_str().unwrap());
        }

        let input = match get_input() {
            Ok(size) => if size <= paths_num {
                    size
                }
                else{
                    println!("Bitte gib eine gültige Option ein");
                    continue;
                },
            Err(string) => {
                if string == "quit" {
                    return Ok(());
                }
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
        env::set_current_dir(dir);
    } 
}