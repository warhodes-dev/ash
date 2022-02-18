use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error;
use std::env;
use std::process;

pub struct Configuration {
    pub prompt: String
}

pub fn run(_config: &Configuration) -> Result<(), Box<dyn error::Error>>{
    loop {
        
        /* === Print Prompt === */
        /* Make prompt path relative to home if possible */
        let home_dir    = env::var("HOME")?;
        let user        = env::var("USER")?;
        let current_dir = env::current_dir()?;
        if let Ok(prompt_path) = current_dir.strip_prefix(&home_dir) {
            print!("{} ~/{}> ", user, prompt_path.display());
        } else {
            print!("{} {}> ", user, current_dir.display());
        }
        stdout().flush()?;

        /* === Read and Parse Input Line === */
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let fields_vec: Vec<&str> = input.trim().split_whitespace().collect();
        let mut fields = fields_vec.into_iter();
        let command = match fields.next() {
            Some(c) => c,
            None => continue,
        };
        let mut args = fields;


        /* === Execute Command === */
        match command {
            "exit" => {
                println!("Thanks for using ash!");
                return Ok(());
            },
            "cd" => {
                match args.next() {
                    Some(c) => { env::set_current_dir(c)?; }
                    None => { env::set_current_dir(home_dir)?; }
                }
            },
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait()?; },
                    Err(e) => { println!("{}", e); },
                }
            }
        }

    }
}
