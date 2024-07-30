use std::process::Command;


fn string_in_vec(string: &String) -> Vec<String>{
    let mut vektor: Vec<String> = Vec::new();
    let mut word = String::new();
    for i in string.chars(){
        if i == '\n'{
            vektor.push(word.clone());
            word = "".to_string();
        }
        else{
            word.push(i);
        }
    }

    vektor
}

pub fn list (path: &String) -> Option<Vec<String>>{
    let output = Command::new("ls")
        .arg(path)
        .output()
        .expect("failed to execute process");   

    if !output.status.success() {
        return None;
    }

    let output_str = if let Ok(string) = String::from_utf8(output.stdout){
        string
    }
    else{
        panic!("ls command didn't return a string");
    };

    Some(string_in_vec(&output_str))
}