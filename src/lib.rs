use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error;
use std::env;
use std::path;
use std::process;

pub struct Configuration {
    pub prompt: String
}

pub fn exit_builtin(errno: i32) {
    println!("Thanks for using ash!");
    process::exit(errno);
}

pub fn cd_builtin() {
}

pub fn run(config: &Configuration) -> Result<(), Box<dyn error::Error>>{
    loop {
        
        /* === Print Prompt === */
        /* Make prompt path relative to home if possible */
        let home_dir    = env::var("HOME")?;
        let user        = env::var("USER")?;
        let current_dir = env::current_dir()?;
        if let Ok(prompt_path) = current_dir.strip_prefix(home_dir) {
            print!("{} ~/{}> ", user, prompt_path.display());
        } else {
            print!("{}:{}>", config.prompt, current_dir.display());
        }
        stdout().flush()?;

        /* === Read Input Line === */
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let fields_vec: Vec<&str> = input.trim().split_whitespace().collect();
        let mut fields = fields_vec.iter();
        let command = match fields.next() {
            Some(c) => c,
            None => continue,
        };

        let mut args = fields;

//      println!("command: {:?}", command);
//      println!("args: {:?}", args);
//
        /* === Built-ins === */

        match *command {
            "exit" => {
                println!("Thanks for using ash!");
                process::exit(0);
            },
            "cd" => {
                env::set_current_dir(args.next().unwrap())?;
                continue;
            },
            _ => ()
        }

        /* === Fork and Exec === */
        let mut child = Command::new(command)
            .args(args)
            .spawn()?;

        child.wait()?;
    }
}
