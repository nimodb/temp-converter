use std::io;

fn main() {
    println!("Welcome to the Temperature Convertor!");
    println!("Please select the Conversion type:");
    println!("1: Fahrenheit to Celsius");
    println!("2: Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if choice == 1 {
        println!("Conver Fahrenheit to Celsius");
    } else if choice == 2 {
        println!("Convert Celsius to Fahrenheit");
    } else {
        println!("Invalid choice, please select 1 or 2");
    }
}
