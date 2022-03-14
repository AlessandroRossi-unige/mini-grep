use std::env;

fn main() {
    println!("Welcome to mini grep!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { panic!("You need two arguments!")};

    let grep_string = &args[1];
    let file_name = &args[2];

    println!("String to grep: {}\nFile path: {}", grep_string, file_name);


}
