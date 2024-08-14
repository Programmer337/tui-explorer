# Tui-explorer
[crates.io](https://crates.io/crates/tui-explorer)
This is a simple and lightweight file explorer for a Command line Interface written completly in Rust
## Installation
### via crates.io
1. Install cargo (see [rust-lang.org](https://www.rust-lang.org/tools/install))
2. run `cargo install tui-explorer`
### build from source
1. install git and Cargo
2. run `git clone https://github.com/Programmer337/tui-explorer.git`
3. `cd tui-explorer`
4. `cargo install --path .`
## Usage
This start this will list all files and subdirs of the current dir. You will have the follwing options:
* enter a number -> navigate to the option asociated with the number
* directly enter a valid path -> navigate to it
* mkdir -> you will be asked a name. Creates a new dir
* a -> This Command will list all (including hidden) files and dir
* cp -> you will be asked for the source and the tarrget to copy
* rm -> enter a file or dir to deleted
* cmd -> enter the command line
* q -> quit the program 
### commandline options and environment variables
To see all directories and files you can use `tui-explorer --list-all`

The programm automatically detects the system language via the `LANG` environment variable which is the standard in UNIX.
The project supports following languages:
* English
* German
* French

The fallback language is English.