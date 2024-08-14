use fluent_templates::{static_loader, Loader};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitStatus;
use std::usize;
use unic_langid::{langid, LanguageIdentifier};

use input::*;

mod input;

static_loader! {
    static LOCALES = {
        locales: "locales",
        fallback_language: "en-US",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(true),
    };
}

pub struct Config {
    list_all: bool,
    language: LanguageIdentifier,
}
impl Config {
    /// reads the config and returns an instance of Config
    /// # Panics
    /// args are not valid unicode
    pub fn read_conf() -> Self {
        // Reading list_all
        let as_string: u8 = env::var("LIST_ALL")
            .unwrap_or_else(|_err| "0".to_string())
            .trim()
            .parse()
            .unwrap_or(0);
        let mut list_all: bool = match as_string {
            0 => false,
            _ => true,
        };

        if env::args().nth(1).unwrap_or("0".to_string()) == *"--list-all" {
            list_all = true;
        };

        // Reading language
        let mut as_string = String::from("");
        // Cuts up Unicode Identifier e.g en_US.UTF-8
        for i in env::var("LANG").unwrap_or("en_US".into()).chars() {
            if i == '.' {
                break;
            }
            as_string += &String::from(i);
        }
        let language = as_string.parse().unwrap_or_else(|err| {
            eprintln!("{}", err);
            langid!("en_US")
        });

        Self { list_all, language }
    }
}

/// opens a file
/// # Errors
/// Command::status returns an error
/// # Panics
/// Any Errors while getting input
fn open_file(file: &Path, lang: &LanguageIdentifier) -> io::Result<ExitStatus> {
    print!("{}", LOCALES.lookup(lang, "ask-for-program"));
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| panic!("{}", LOCALES.lookup(lang, "input-error")));

    Command::new(input.trim()).arg(file).status()
}

/// prints out all directorys and returns the total size of elements
/// # Panics
/// an element is not valid unicode
fn print_dirs(paths: &Vec<PathBuf>) -> usize {
    println!("0: ..");
    let mut i: usize = 0;
    for path in paths {
        i += 1;
        println!("{}. {}", i, path.file_name().unwrap().to_str().unwrap());
    }
    i
}

/// filters out all elemts that start with '.'
/// # Panics
/// an element is not valid unicode
fn filter_elements(elements: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = vec![];
    for i in elements {
        if !i.file_name().unwrap().to_str().unwrap().starts_with(".") {
            result.push(i);
        }
    }
    result
}

/// rekursive function, that copies a whole directory
fn copy_dir(from: &Path, to: &Path, lang: &LanguageIdentifier) {
    if from.is_dir() {
        if !to.exists() {
            fs::create_dir_all(to).unwrap_or_else(|err| handle_err(err, lang));
        }
        let paths = match fs::read_dir(from) {
            Ok(paths) => paths,
            Err(err) => {
                handle_err(err, lang);
                return;
            }
        };
        for dir in paths {
            copy_dir(
                dir.as_ref().unwrap().path().as_path(),
                &to.join(dir.unwrap().file_name()),
                lang,
            );
        }
    } else {
        fs::copy(from, to).unwrap_or_else(|err| {
            eprintln!("{err}");
            0
        });
    }
}

/// takes io::Error and prints a specific Message to stderr
fn handle_err(err: io::Error, lang: &LanguageIdentifier) {
    eprintln!(
        "{}",
        match err.kind() {
            io::ErrorKind::NotFound => LOCALES.lookup(lang, "err-not-found"),
            io::ErrorKind::PermissionDenied => LOCALES.lookup(lang, "err-permission-denied"),
            io::ErrorKind::AlreadyExists => LOCALES.lookup(lang, "err-already-exists"),
            _ => err.kind().to_string(),
        }
    )
}

/// runs the programm
/// # Panics
/// current_dir is unvalid or the user lacks permission
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

        let input = match Input::get_input(&config.language) {
            Ok(input) => match input {
                Input::Choose(size) => {
                    if size > paths_num {
                        eprintln!("{}", LOCALES.lookup(&config.language, "no-valid-option"));
                        continue;
                    }
                    size
                }
                Input::DirName(name) => {
                    if Path::new(&name.trim()).is_dir() {
                        env::set_current_dir(name.trim())
                            .unwrap_or_else(|err| handle_err(err, &config.language));
                    } else {
                        println!(
                            "{}",
                            if let Err(err) = open_file(Path::new(&name), &config.language) {
                                if err.kind() == io::ErrorKind::NotFound {
                                    LOCALES.lookup(&config.language, "program-doesnt-exist")
                                } else {
                                    err.kind().to_string()
                                }
                            } else {
                                String::from("")
                            }
                        );
                    }
                    continue;
                }
                Input::Copy(from, to) => {
                    copy_dir(Path::new(&from), Path::new(&to), &config.language);
                    continue;
                }
                Input::NewDir(name) => {
                    fs::create_dir(name).unwrap_or_else(|err| handle_err(err, &config.language));
                    continue;
                }
                Input::Rm(name) => {
                    let as_path = Path::new(name.trim());
                    if as_path.is_dir() {
                        fs::remove_dir_all(name.trim())
                            .unwrap_or_else(|err| handle_err(err, &config.language));
                    } else {
                        fs::remove_file(name.trim())
                            .unwrap_or_else(|err| handle_err(err, &config.language));
                    }
                    continue;
                }
                Input::Command(cmd) => {
                    if let Err(err) = Command::new("sh").arg("-c").arg(cmd).status() {
                        handle_err(err, &config.language);
                    }
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
                eprintln!("{}", LOCALES.lookup(&config.language, "no-root-dir"));
                continue;
            }
        } else {
            Path::new(&paths[input - 1])
        };
        if dir.is_file() {
            println!(
                "{}",
                if let Err(err) = open_file(dir, &config.language) {
                    if err.kind() == io::ErrorKind::NotFound {
                        LOCALES.lookup(&config.language, "program-doesnt-exist")
                    } else {
                        err.kind().to_string()
                    }
                } else {
                    String::from("")
                }
            );
            continue;
        }
        env::set_current_dir(dir).unwrap_or_else(|err| handle_err(err, &config.language));
    }
}
