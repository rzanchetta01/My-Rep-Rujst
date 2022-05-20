use regex::Regex;

fn main() {
    /* 
        Indentifies palindromes in a given phrase
    */

    let mut s: String = String::from("A man, a plan, a canal: Panama");

    let filter = Regex::new(r"[^A-Za-z]").unwrap();

    s = s.trim().to_lowercase();
    s = filter.replace_all(&s, "").to_string();

    let mut clone_s = s.clone();

    let mut str: String = String::new();

    for _i in 0..s.len() {

        str.push(clone_s.pop().unwrap())
    }

    println!("{}", s);
    println!("{}", str);

    if s.eq(&str) {
        println!("True");
    } else {
        println!("False");
    }
}

