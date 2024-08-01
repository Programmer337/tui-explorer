use std::env;
use std::io;
use std::path::Path;
use std::usize;


fn get_input() -> Result<usize, String>{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");
    let input: usize = if let Ok(usize) = input.trim().parse(){
        usize
    }
    else{
        return Err("Eingabe ist keine Zahl".to_string());
    };
    Ok(input)
}

pub fn run() -> Result<(), String>{

    loop{
        let dir = env::current_dir().unwrap().clone();
        let mut dir = dir.as_path();

        let mut paths = dir.read_dir().unwrap();

        println!("0: ..");
        let mut i: u32 = 0;
        for path in paths{
            i += 1;
            println!("{}. {}", i, path.unwrap().path().file_name().unwrap().to_str().unwrap());
        }

        let input = match get_input() {
            Ok(usize) => usize,
            Err(string) => return Err(string),
        };

        let mut paths = dir.read_dir().unwrap();
        let as_os_string = paths.nth(input-1).unwrap().unwrap().file_name();
        dir = if input == 0{
            dir.parent().unwrap()
        }
        else {
            Path::new(&as_os_string)
        };
        env::set_current_dir(dir);
    }

    Ok(())
}