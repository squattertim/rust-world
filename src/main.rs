use std::env;
use std::io;
use slug::slugify;

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [ref name] => {
            println!("Running '{}'. Please, pass some arguments!", name);
        },
        [_, ref string] => {

            println!("Enter your input:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match string.as_str() {
                "lowercase" => lowercase(input),
                "uppercase" => uppercase(input),
                "nospaces" => nospaces(input),
                "slugify" => slug(input),
                _ => {
                    println!("Error: unknown command");
                },
            }
        },
        _ => {
            println!("Too many args!");
        }
    }
}

fn print_result(result: String) {
    println!("Result: {}", result);
}

fn uppercase(string: String) {
    print_result(string.to_uppercase());
}

fn lowercase(string: String) {
    print_result(string.to_lowercase());
}

fn nospaces(string: String) {
    print_result(string.replace(" ", ""));
}

fn slug(string: String) {
    print_result(slugify(string));
}
