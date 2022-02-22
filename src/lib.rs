use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error;
use std::env;
use std::process;

pub struct Configuration {
    pub prompt: String
}

pub enum Flow { Cond, Pipe, }

pub struct Statement <'a> {
    command:    &'a str,
    args:       Vec<&'a str>,
    file_in:    Option<&'a str>, 
    file_out:   Option<&'a str>,
    flow:       Option<Flow>,
    trunc:      bool,
//  background: bool,
    child:      Option<process::Child>,
}

fn parse_input(input: &str) -> Result<Vec<Statement>, Box<dyn error::Error>> {

    /* Split into each seperate exec (stmt) */
    let fields = input.split_inclusive(&['|', ';', '?'][..])
                      .collect::<Vec<&str>>();

    let mut stmts: Vec<Statement> = Vec::new();
    for field in fields {

        /* Parse flow operation and trim */
        let stmt_flow = match field.chars().last().unwrap() {
            '|' => Some(Flow::Pipe),
            '?' => Some(Flow::Cond),
             _  => None,
        };
        let field_trim = field.trim_end_matches(&['|', '?', ';'][..]);

        /* Parse file redirection and trim */
        let mut stmt_trunc = false;
        let mut  stmt_fout: Option<&str> = None;
        let mut   stmt_fin: Option<&str> = None;
        let mut field_iter = field_trim.split_whitespace().peekable();
        while let Some(item) = field_iter.next() {
            match item {
                "<" => {
                    stmt_fin = field_iter.next();
                },
                ">" => {
                    stmt_fout = field_iter.next();
                    stmt_trunc = true;
                },
                "+>" => {
                    stmt_fout = field_iter.next();
                },
                _ => {}
            }
        }

        /* Parse command and arguments */
        let mut stmt_body: Vec<&str> = Vec::new();
        let field_trim_split = field_trim.split(&['<', '>'][..])
                                         .collect::<Vec<&str>>();
        if let Some(s) = field_trim_split.first() {
            stmt_body = (*s).split_whitespace()
                            .collect::<Vec<&str>>();
        }
        if stmt_body.is_empty() {
            return Err("Empty Arguments".into())
        }
        let stmt_args = stmt_body.split_off(1);
        let stmt_command = stmt_body.first().unwrap();

        /* Build statement and push to return vector */
        stmts.push(
            Statement {
                command:    stmt_command,
                args:       stmt_args,
                file_in:    stmt_fin,
                file_out:   stmt_fout,
                flow:       stmt_flow,
                trunc:      stmt_trunc,
                child:      None
            });
    }
    Ok(stmts)
}

pub fn run(_config: &Configuration) -> Result<(), Box<dyn error::Error>> {
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

        let statements = parse_input(&input)?;

        for (i, stmt) in statements.iter().enumerate() {
            println!("Statement: {}", i);
            println!("  Command: {}", stmt.command);
            println!("    Stdin: {:?}", stmt.file_in);
            println!("   Stdout: {:?}", stmt.file_out);
            println!("    Trunc: {}", stmt.trunc);
            print!  ("     Flow: ");
                if stmt.flow.is_none() {
                    println!("None");
                } else {
                    match stmt.flow.as_ref().unwrap() {
                        Flow::Pipe => { println!("Pipe"); },
                        Flow::Cond => { println!("Cond"); },
                    }
                }
            println!("     Args: {:?}\n", stmt.args);
        }

        /* === Execute Command === */
        for stmt in statements {
            match stmt.command {
                "exit" => {
                    println!("Thanks for using ash!");
                    return Ok(());
                },
                "cd" => {
                    match stmt.args.first() {
                        Some(c) => { println!("Moving to {:?}", c); env::set_current_dir(c)?; }
                        None => { env::set_current_dir(&home_dir)?; }
                    }
                },
                command => {
                    let child = Command::new(command)
                        .args(stmt.args.iter().copied())
                        .spawn();

                    match child {
                        Ok(mut child) => { child.wait()?; },
                        Err(e) => { println!("Error: {}", e); },
                    }
                }
            }
        }
    }
}
