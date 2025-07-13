use std::io::{self, BufRead};

enum Command {
    PUSH(String),
    TOP(),
    SIZE(),
    EMPTY(),
    POP()
}


struct Stack {
    buffer: Vec<String>
}

impl Stack {

    pub fn new() -> Self {
        Stack{buffer: Vec::new()}
    }

    pub fn perform(&mut self, command: &Command) {
        match command {
            Command::PUSH(value) => {
                self.buffer.push(value.to_string());
            }
            Command::TOP() => {
                if self.buffer.len() > 0 {
                    println!("{}", self.buffer[self.buffer.len() - 1]);
                }
                else {
                    println!("-1");
                }
            }
            Command::SIZE() => {
                println!("{}", self.buffer.len());
            }
            Command::EMPTY() => {
                if self.buffer.len() == 0 {
                    println!("1")
                }
                else {
                    println!("0")
                }
            }
            Command::POP() => {
                let value = self.buffer
                    .pop()
                    .unwrap_or("-1".to_string());
                println!("{}", value)
            }
            cmd => { panic!("Unknown command") }
        }
    }
}


pub fn main() {
    let mut size_string = String::new();
    io::stdin().read_line(&mut size_string);
    let size: usize = size_string
        .trim()
        .parse()
        .unwrap();

    let commands: Vec<Command> = io::stdin()
        .lock()
        .lines()
        .take(size)
        .map(|line| {
            let binding = line.unwrap();
            let mut parts = binding.split_whitespace();
            match parts.next().unwrap() {
                "push" => { Command::PUSH(parts.next().unwrap().to_string()) }
                "pop" => { Command::POP() }
                "top" => { Command::TOP() }
                "size" => { Command::SIZE() }
                "empty" => { Command::EMPTY() }
                cmd => { panic!("Unknown command"); }
            }
        })
        .collect();

    let mut stack = Stack::new();
    commands.iter()
        .for_each(|command| stack.perform(command));
}