use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Renan";

    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else {
        println!("mando o papo, é pra fazer oq?");
    }
}
