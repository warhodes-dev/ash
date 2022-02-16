extern crate clap;
use clap::{Arg, App};

use ash::{Configuration, run};

fn main() {
    let args = App::new("aShell")
                    .version("0.1")
                    .author("warhodes <warhodes@gmail.com>")
                    .about("Lightweight shell written in rust")
                    .arg(Arg::with_name("prompt")
                        .short("p")
                        .long("prompt")
                        .help("basic ascii prompt (No PS1 support)")
                        .default_value("ash"))
                    .get_matches();

    let config = Configuration {
        prompt: args.value_of("prompt").unwrap().to_string()
    };

    if let Err(e) = run(&config) {
        println!("ash encountered an error: {}", e);
    }
}
