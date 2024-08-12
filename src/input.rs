use std::io::{self, Write};

use fluent_templates::Loader;

use crate::LOCALES;

/// stores the user's input
pub enum Input {
    Choose(usize),
    ListAll,
    DirName(String),
    Copy(String, String),
    NewDir(String),
    Rm(String),
    Command(String),
    Quit,
}

impl Input {

    /// reads input and returns it in a Result
    /// # Panics
    /// Input Errors
    pub fn get_input(lang: &unic_langid::LanguageIdentifier) -> Result<Self, String> {
        print!("{} ", LOCALES.lookup(lang, "choose-option"));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(&LOCALES.lookup(lang, "input-error"));

        match input.trim() {
            "q" => Ok(Self::Quit),
            "cp" => {
                let mut from = String::from("");
                print!("{} ", LOCALES.lookup(lang, "from"));
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut from).expect(&LOCALES.lookup(lang, "input-error"));

                let mut to = String::from("");
                print!("{} ", LOCALES.lookup(lang, "to"));
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut to).expect(&LOCALES.lookup(lang, "input-error"));

                Ok(Self::Copy(from.trim().to_string(), to.trim().to_string()))
            }
            "mkdir" => {
                input = String::from("");
                print!("Name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect(&LOCALES.lookup(lang, "input-error"));
                Ok(Self::NewDir(input.trim().to_string()))
            }
            "rm" => {
                input = String::from("");
                print!("Name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect(&LOCALES.lookup(lang, "input-error"));

                Ok(Self::Rm(input.trim().to_string()))
            }
            "a" => Ok(Self::ListAll),
            "cmd" => {
                input = "".to_string();
                print!("$: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect(&LOCALES.lookup(lang, "input-error"));
                
                Ok(Self::Command(input))
            }
            _ => {
                if let Ok(usize) = input.trim().parse() {
                    Ok(Self::Choose(usize))
                } else {
                    Ok(Self::DirName(input.trim().to_string()))
                }
            }
        }
    }
}
