
//using strum_macro for display enum in println
#[derive(strum_macros::Display)]
enum State {
    Idle,
    Running,
    Stop,
    Walking,
}

fn main() {
    //data types
    let string_var: String = "batata".to_string();
    let integer_assingned_64: i64 = -1000;
    let integer_unsinged_64: u64 = 1000;
    let float_64: f32 = 1000.00;
    let bool_var: bool = true;
    let char_var: char = '😻';
    let tuple_var: (i64, u64, f32) = (integer_assingned_64, integer_unsinged_64, float_64);
    let mut count: u64 = 0;
    let mut _object_state: State = State::Idle;
    const MAX_NUMBER: u8 = 10;


    println!("{string_var} {char_var}");

    if bool_var != false {
        println!("{bool_var}");
        println!("{}{}{}", tuple_var.0, tuple_var.1, tuple_var.2);
    }

    //infinte loop
    loop {
        count += 1;

        if count == 100 {
            println!("{count}");
            break;
        }
    }

    count = 0;
    //while loop
    while count <= 1000 {
        count += 1;
    }

    println!("{count}");

    count = 0;
    //for loop
    for n in 0..10000 {
        count += n;
    }

    println!("{count}");

    for n in 0..MAX_NUMBER {
        if n == 0 {
            _object_state = State::Walking;
        } else if n == 1 {
            _object_state = State::Running;
        } else if n == 2 {
            _object_state = State::Stop;
        } else {
            _object_state = State::Idle;
        }

        println!("{_object_state}");
    }
}
