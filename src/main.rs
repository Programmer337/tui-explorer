mod filesystem;

fn main() {
    if let Some(vektor) = filesystem::list(&"/root".to_string()){
        for i in vektor{
            println!("{i}");
        }
    }
    else {
        print!("keine Rückgabe");
    }
}