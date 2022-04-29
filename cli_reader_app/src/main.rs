use std::env;
use std::process;
use std::io;

use cli_reader_app::printer_string;
use cli_reader_app::printer_vector;
use cli_reader_app::read_text;
use cli_reader_app::search_in_text;
use cli_reader_app::Config;

fn main() {
    const NUM_ARGS: usize = 2;
    
    let mut args: Vec<String> = env::args().collect();
    loop {
        
        let config: Config = Config::new(&args.clone(), NUM_ARGS).unwrap_or_else(|err| {
            println!("problem parsing : {err}");
            process::exit(1);
        });

        println!("Searching for in the path: {}", config.filepath);
        let mut text: String = String::new();

        if let Err(e) = read_text(config.clone()) {
            println!("aplication error: {}", e);
        } else if let Ok(e) = read_text(config.clone()) {
            text = e.clone();

            let filter: String = "-donotread".trim().to_string();

            if !args.clone().contains(&filter) {
                printer_string(e);
            }
        }

        if let Err(e) = search_in_text(&text, &config.searchkeyword) {
            println!("aplication error: {}", e);
        } else if let Ok(e) = search_in_text(&text, &config.searchkeyword) {
            printer_vector(e);
        }

        args = user_input(args);
    }
}

fn user_input(mut args : Vec<String>) -> Vec<String> {

    let mut input_command = String::new();
    io::stdin().read_line(&mut input_command).expect("ERROR READING NEW COMMAND");

    args.drain(1..);
    let mut new_args: Vec<String> = input_command.split_whitespace().map(|str| str.to_string()).collect();
    args.append(&mut new_args);
    return args;
}
