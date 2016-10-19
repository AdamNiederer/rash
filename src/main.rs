extern crate readline;
use std::string::String;
use std::vec::Vec;
use std::str;
use std::process::Command;
use std::process::Stdio;
use std::fs::File;
use std::path::Path;
//use std::path::Path;
//use std::env::set_current_dir;
use std::env::current_dir;

mod cmd;

fn main() {
    loop {
        let shell_prompt = format!("[{}]$ ", current_dir().unwrap().file_stem().unwrap().to_str().expect(""));
        let input = readline::readline(&shell_prompt).unwrap();
        readline::add_history(input.as_str());
        match cmd::tokenize(&input) {
            Ok(s) => {
                cmd::RashCmd::new(&s[0])
                    .redirect_to(File::create(Path::new("bar.txt")).expect(""))
                    .args(&s[1..])
                    .eval()
                    .expect("");
            },
            Err(s) => println!("rash: error: {}", s)
        }
    }
}
