use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

mod interpreter;

fn read_file(path: &String) -> String {
    let mut code_buffer = String::new();

    let file = match File::open(path) {
        Err(reason) => {
            panic!("Could not open {}:{}", path, reason)
        }
        Ok(file) => file,
    };

    let mut fin = BufReader::new(file);
    fin.read_to_string(&mut code_buffer).unwrap();
    code_buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    interpreter::interpreter(&read_file(&args[1]))
}
