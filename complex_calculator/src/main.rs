//use rand::Rng;
//use pgp::util;
use std::io;

fn main() {
    loop {
        println!("COMPLEX CALCULATOR FOR LEAVE TYPE  ( EXIT ) ");
        println!("PLEASE INPUT YOUR MATH EXPRESSION");
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("CALCULATOR ERROR");

        if input.trim().to_lowercase().eq("exit") {
            println!("THANKS FOR USING ME!");
            break;
        }

        math_analiser(input.to_lowercase().to_string());
    }
}

fn math_analiser(expression: String) {
    //let parenthesis = ['(', ')'];

    let chars: Vec<char> = expression.chars().collect();

    for ch in chars {
        if ch == '(' {
            let final = chars.c
        }
    }
    

    /*for ch in &mut chars {
        if parenthesis.contains(ch) {
            for ch_p in &mut chars {
               /*  if ch_p == ')' {
                    let partial_math = &chars.drain(1..2);
                }*/
            }

        }
    }*/
}
