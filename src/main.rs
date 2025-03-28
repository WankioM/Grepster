use std::env;
use colored::*;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new (&args).unwrap_or_else(|err| {
        println!("\n problem parsing arguments : {} ", err);
        process::exit(1)
    });
    

    println!("\n Searching for {}", config.query.bright_cyan());
    println!("In file {} ", config.file.bright_cyan());

   run(config);
}


fn run(config: Config)-> Result <(), Box<dyn Error>>{
    
    let contents = fs::read_to_string(&config.file)?;
    
    println!("{}", format!(" \n With text: \n {}", contents).bright_green());
    Ok(())
}

struct Config {
    query:String,
    file: String,
}

impl Config {

fn new( args: &[String]) -> Result<Config, &str> {

    if args.len() < 3 {
        println!("{}", "\n Not enough arguments!".red());
        panic!("panicking!");
    }

    let query = args[1].clone();
    let file = args[2].clone();

    Ok(Config { query, file})
}

}