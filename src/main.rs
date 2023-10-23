use std::env;
use std::io;
use std::error::Error;
use slug::slugify;
use csv;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_args(args) {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("{}", e)
    };
}

fn parse_args(args: Vec<String>) -> Result<String, Box<dyn Error>> {
    return match args.as_slice() {
        [ref name] => {
            Err(Box::try_from(format!("Running '{}'. Please, pass some arguments!", name)).unwrap())
        }
        [_, ref string] => {
            println!("Enter your input:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match string.as_str() {
                "lowercase" => lowercase(input),
                "uppercase" => uppercase(input),
                "nospaces" => no_space(input),
                "slugify" => slug(input),
                "csv" => parse_csv(input),
                _ => {
                    Err(Box::try_from("Error: unknown command").unwrap())
                }
            }
        }
        _ => {
            Err(Box::try_from("Too many args!").unwrap())
        }
    };
}

fn uppercase(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.to_uppercase())
}

fn lowercase(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.to_lowercase())
}

fn no_space(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.replace(" ", ""))
}

fn slug(string: String) -> Result<String, Box<dyn Error>> {
    Ok(slugify(string))
}

fn parse_csv(string: String) -> Result<String, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(string.as_bytes());
    let mut printable = String::new();
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        printable += format!("{:?}", record).as_mut_str();
    }
    Ok(printable)
}
