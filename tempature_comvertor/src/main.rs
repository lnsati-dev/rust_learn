use std::io::{self, Write};

fn convertor() {
    print!("Enter the Tempature  ");
    io::stdout().flush().unwrap();
}

fn process_imput(letter: &str) {
    println!("Selected '{}' ", letter);
    convertor();
}
fn main() {
    println!("Tempature Convertor");
    loop {
        print!("press f to input Fahrenheit value, c for Celsius or e to end  ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");
        let letter = input.trim().to_lowercase();

        match letter.as_str() {
            "f" => process_imput("f"),
            "c" => process_imput("c"),
            "e" => {
                println!("Ending .....");
                break;
            }
            _ => println!("please try again"),
        }
    }
}
