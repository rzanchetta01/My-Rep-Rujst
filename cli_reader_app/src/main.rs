use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!(
        "Searchinf for {} in the path: {}",
        config.filename, config.filepath
    );
    
    let read_result: String = read_text(config);
    println!("{read_result}");
}

struct Config {
    filename: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename: String = args[1].clone();
        let filepath: String = args[2].clone();

        return Config { filename, filepath };
    }
}

fn read_text(search: Config) -> String {
    let contents: String = fs::read_to_string(search.filepath).expect("ERROR READING THE FILE");

    return contents;
}
