use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In the file {}\n",config.filename);

    if let Err(e) = minigrep::run(config){
        eprint!("Application error: {}", e);
        process::exit(1)
    }
}