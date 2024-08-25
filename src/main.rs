use std::io;

fn main() {
    println!("Welcome to the Temperature Convertor!");
    let choice = get_conversion_choice();

    match choice {
        1 => {
            let temp = get_temperature("Fahrenheit");
            let celsius = fahrenheit_to_celsius(temp);

            println!("Converting Fahrenheit to Celsius...");
            println!("{temp}°F is equal to {celsius:.2}°C");
        }
        2 => {
            let temp = get_temperature("Celsius");
            let fahrenheit = celsius_to_fahrenheit(temp);

            println!("Converting Celsius to Fahrenheit...");
            println!("{temp}°C is equal to {fahrenheit:.2}°F");
        }
        _ => println!("Invalid choice, please select 1 or 2"),
    }
}

fn get_conversion_choice() -> u32 {
    println!("Please select the Conversion type:");
    println!("1: Fahrenheit to Celsius");
    println!("2: Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            1
        }
    }
}

fn get_temperature(scale: &str) -> f64 {
    println!("Enter temperature in {scale}:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            0.0
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
