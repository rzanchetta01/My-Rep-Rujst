mod command_structure;
mod config;
mod util;

use config::Config;
use std::process;

fn main() {
    let args: Vec<String> = util::input_to_vector();

    if args.contains(&command_structure::DebugCommannds::_DebugMode.to_string()) {
        if args.contains(&command_structure::DebugCommannds::_Loop.to_string()) {
            dev_mode(args.clone());
        }

        println!("for debug please use -debugMode and -loop");
    }
}

fn dev_mode(args: Vec<String>) {
    loop {
        if args.contains(&command_structure::DebugCommannds::_Exit.to_string()) {
            break;
        }

        let configuration: Config = Config::new(&args.clone()).unwrap_or_else(|err| {
            println!("ERROR : {err}");
            process::exit(1);
        });

        util::printer_vector(configuration.querry);
    }
}
