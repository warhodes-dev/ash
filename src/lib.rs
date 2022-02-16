use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error;
use std::env;
use std::path;
use std::process;

pub struct Configuration {
    pub prompt: String
}

pub fn run(config: &Configuration) -> Result<(), Box<dyn error::Error>>{
    loop {
        
        /* === Print Prompt === */
        /* Make prompt path relative to home if possible */
        let home_dir = env::var("HOME")?;
        let current_dir = env::current_dir()?;
        if let Ok(prompt_path) = current_dir.strip_prefix(home_dir) {
            print!("{}:~/{}>", config.prompt, prompt_path.display());
        } else {
            print!("{}:{}>", config.prompt, current_dir.display());
        }
        stdout().flush()?;

        /* === Read Input Line === */
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut fields = input.trim().split_whitespace();
        let command = match fields.next() {
            Some(c) => c,
            None => continue,
        };

        let args = fields;

//      println!("command: {:?}", command);
//      println!("args: {:?}", args);
//
        /* === Built-ins === */

        if command == "exit" {
            process::exit(0);
        }

        /* === Fork and Exec === */
        let mut child = Command::new(command)
            .args(args)
            .spawn()?;

        child.wait()?;
    }
}
