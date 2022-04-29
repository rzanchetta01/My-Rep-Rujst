use std::error::Error;
use std::fs;
use std::process;

#[derive(Clone)]
pub struct Config {
    pub filepath: String,
    pub searchkeyword: Vec<String>,
}

impl Config {
    pub fn new(args: &[String], num_args: usize) -> Result<Config, &str> {
        let exiter: String = "-quit".trim().to_string();
        if args.clone().contains(&exiter) {
            process::exit(0);
        }

        if args.len() < num_args - 2 {
            return Err(
                "not enough arguments\nthe correct structure is -> cargo run (file path) (querry) ",
            );
        }

        let filepath: String = args[1].clone();
        let mut searchkeyword: Vec<String> = Vec::new();

        for i in 2..args.len() {
            searchkeyword.push(args[i].clone());
        }

        return Ok(Config {
            filepath,
            searchkeyword,
        });
    }
}

pub fn read_text(search: Config) -> Result<String, Box<dyn Error>> {
    let contents: String = fs::read_to_string(search.filepath)?;
    return Ok(contents);
}

pub fn search_in_text<'a>(content: &'a str, querry: &Vec<String>) -> Result<Vec<&'a str>, String> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in content.lines() {
        for keyword in querry {
            if line.contains(keyword) {
                if !result.contains(&line) {
                    result.push(line);
                }
            }
        }
    }

    if result.len() < 1 {
        return Err("did not found anything or does not have a querry keyword"
            .trim()
            .to_string());
    }

    return Ok(result);
}

pub fn printer_string(content: String) {
    println!("{content}")
}

pub fn printer_vector(content: Vec<&str>) {
    println!("\n{:?}\n", content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_valid_text() {
        let filepath: [String; 2] = [
            "target\\debug\\cli_reader_app.exe".trim().to_string(),
            "poem.pdf".trim().to_string(),
        ];

        let sut: Config = Config::new(&filepath, 2).unwrap_or_else(|err| panic!("{}", err));

        if let Err(e) = read_text(sut) {
            panic!("aplication error: {}", e);
        }
    }

    #[test]
    #[should_panic]
    fn read_invalid_text() {
        let filepath: [String; 2] = [
            "target\\debug\\cli_reader_app.exe".trim().to_string(),
            "poem.pdfs".trim().to_string(),
        ];

        let sut: Config = Config::new(&filepath, 2).unwrap_or_else(|err| panic!("{}", err));

        if let Err(e) = read_text(sut) {
            panic!("aplication error: {}", e);
        }
    }

    #[test]
    fn search_valid_in_text() {
        let filepath: [String; 3] = [
            "target\\debug\\cli_reader_app.exe".trim().to_string(),
            "poem.pdf".trim().to_string(),
            " É ".trim().to_string(),
        ];
        let mut text: String = String::new();

        let sut: Config = Config::new(&filepath, 2).unwrap_or_else(|err| panic!("{}", err));
        println!("{:?}", &filepath);

        if let Ok(e) = read_text(sut.clone()) {
            text = e.clone();
        }

        if let Err(e) = search_in_text(&text, &sut.searchkeyword) {
            panic!("aplication error: {}", e);
        }
    }

    #[test]
    #[should_panic]
    fn search_invalid_in_text() {
        let filepath: [String; 3] = [
            "target\\debug\\cli_reader_app.exe".trim().to_string(),
            "poem.pdf".trim().to_string(),
            "ÉAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".trim().to_string(),
        ];

        let mut text: String = String::new();

        let sut: Config = Config::new(&filepath, 2).unwrap_or_else(|err| panic!("{}", err));

        if let Ok(e) = read_text(sut.clone()) {
            text = e;
        }

        if let Err(e) = search_in_text(&text, &sut.searchkeyword) {
            panic!("aplication error: {}", e);
        }
    }
}
