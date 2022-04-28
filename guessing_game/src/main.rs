use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("WELCOME TO THE GUESS NUMBER");

    let goal_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input: String = String::new();
        println!("INPUT YOUR GUESS");
        io::stdin().read_line(&mut input).expect("READ ERROR");

        let user_number: u32 = match input.trim().parse::<u32>(){
            Ok(num) => num,
            Err(..) => {
                println!("ERROR, TYPE A NUMBER BETWEEN 0 TO 101");
                continue;
            }, 
        };

        match user_number.cmp(&goal_number) {
            Ordering::Less => println!("TO SMALL!"),
            Ordering::Equal => {
                println!("CORRECT!");
                break;
            }
            Ordering::Greater => println!("TO BIG!"),
        }
    }
}
