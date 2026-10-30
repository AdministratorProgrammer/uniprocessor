use std::io;

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("fatal error: failed to read user input");

    input
}

