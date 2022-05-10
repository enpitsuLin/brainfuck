use std::{collections::HashMap, io};

fn execute(
    code: &Vec<char>,
    code_pointer: &mut usize,
    memory: &mut Vec<u8>,
    mem_pointer: &mut usize,
    brackets_cache: &mut HashMap<usize, usize>,
) {
    match code[*code_pointer] {
        '>' => *mem_pointer += 1,
        '<' => *mem_pointer -= 1,
        '+' => memory[*mem_pointer] = memory[*mem_pointer].wrapping_add(1),
        '-' => memory[*mem_pointer] = memory[*mem_pointer].wrapping_sub(1),
        '.' => {
            print!("{}", memory[*mem_pointer] as char);
        }
        ',' => {
            let mut tmp_str = String::new();
            io::stdin().read_line(&mut tmp_str).unwrap();
            memory[*mem_pointer] = tmp_str.chars().next().unwrap() as u8;
        }
        '[' => {
            if memory[*mem_pointer] == 0 {
                *code_pointer = *brackets_cache.get(code_pointer).unwrap();
            }
        }
        ']' => {
            if memory[*mem_pointer] != 0 {
                *code_pointer = *brackets_cache.get(code_pointer).unwrap();
            }
        }
        _ => (),
    }
    *code_pointer += 1;
}

fn fill_brackets_cache(code: &Vec<char>, brackets_cache: &mut HashMap<usize, usize>) {
    let mut stack = Vec::new();
    for index in 0..code.len() {
        match code[index] {
            '[' => stack.push(index),
            ']' => {
                let left = match stack.pop() {
                    Some(left) => left,
                    _ => panic!("Unmatched brackets at position {}", index),
                };
                brackets_cache.insert(left, index);
                brackets_cache.insert(index, left);
            }
            _ => (),
        }
    }
}

pub fn interpreter(code_buffer: &String) {
    let mut memory: Vec<u8> = vec![0; 10000];
    let mut mem_pointer: usize = 0;

    let code: Vec<char> = code_buffer.chars().collect();
    let mut code_pointer: usize = 0;

    let mut brackets_cache = HashMap::new();
    fill_brackets_cache(&code, &mut brackets_cache);

    loop {
        execute(
            &code,
            &mut code_pointer,
            &mut memory,
            &mut mem_pointer,
            &mut brackets_cache,
        );
        if code_pointer == code.len() {
            break;
        }
    }
}
