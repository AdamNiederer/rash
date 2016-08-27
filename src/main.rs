extern crate readline;
extern crate glob;
use glob::glob;
use std::string::String;
use std::vec::Vec;
use std::process::Command;

fn main() {
    loop {
        let input = readline::readline("$ ").unwrap();
        readline::add_history(input.as_str());
        match rash_eval(&input) {
            Ok(s) => {
                let mut p = Command::new(&s[0]).args(&s[1..]).spawn().expect("");
                println!("{:?}", p.wait().expect("").success());
            },
            Err(s) => println!("rash: error: {}", s)
        }
    }
}

fn rash_eval(input: &String) -> Result<Vec<String>, String> {
    // Parses a shell input, deglobs it, and returns a vector of strings
    let mut tokens = Vec::new();
    for word in input.split_whitespace() {
        if word.contains("*") {
            let mut globbed = false;
            for glob in glob(word).expect("Failed to parse wildcard") {
                let path = glob.expect("Failed to parse glob").to_str().unwrap().to_string();
                globbed = true;
                tokens.push(String::from(path));
            }
            if !globbed {
                return Err(format!("Failed to expand glob: {}", word));
            }
        } else {
            tokens.push(String::from(word));
        }
    }
    return Ok(tokens)
}
