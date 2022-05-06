use std::env;
use std::process;
use command_line::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = command_line::read_txt(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


