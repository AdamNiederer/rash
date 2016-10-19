extern crate glob;

use std::string::String;
use std::process::{Stdio, Command};
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;

pub struct RashCmd {
    cmd: Command,
    name: String,
    args: Vec<String>,
    stdin: Stdio,
    stdout: Stdio,
    stderr: Stdio,
    pred: Option<u8>
}

impl RashCmd {
    pub fn new(name: &String) -> RashCmd {
        return RashCmd {
            cmd: Command::new(name.clone()),
            name: name.clone(),
            args: Vec::new(),
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
            pred: None
        }
    }

    // pub fn pipe_to(&mut self, to: &mut RashCmd) -> &mut RashCmd {
    //     unsafe {
    //         to.stdin = Stdio::from_raw_fd(self.stdout.as_raw_fd());
    //     }
    //     return self
    // }

    pub fn redirect_to(&mut self, to: File) -> &mut RashCmd {
        unsafe {
            self.stdout = Stdio::from_raw_fd(to.as_raw_fd());
        }
        return self
    }

    pub fn args(&mut self, args: &[String]) -> &mut RashCmd {
        for arg in args {
            self.args.push(arg.clone());
        }
        return self
    }

    pub fn eval(&mut self) -> Result<(), String> {
        // Executes a command, with arguments

        self.cmd.spawn()
            .expect("Failed to spawn")
            .wait()
            .expect("Failed to exec");

        return Ok(())
    }
}

// pub fn parse(tokens: Vec<&str>) -> RashCmd {
//     // Takes a list of tokens, and turns it into predicated and piped RashCmds.
//     // TODO: Implement
// }

pub fn tokenize(input: &str) -> Result<Vec<String>, String> {
    // Parses a shell input, deglobs it, and returns a vector of strings
    let mut tokens = Vec::new();
    for word in input.split_whitespace() {
        if word.contains("*") {
            let mut globbed = false;
            for glob in glob::glob(word).expect("Failed to parse wildcard") {
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
