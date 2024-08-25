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
        println!("Enter temperature in Fahrenheit:");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };

        let celsius = fahrenheit_to_celsius(temp);
        println!("Converting Fahrenheit to Celsius...");
        println!("{temp}°F is equal to {celsius:.2}°C");
    } else if choice == 2 {
        println!("Enter temperature in Celsius:");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };

        let fahrenheit = celsius_to_fahrenheit(temp);
        println!("Converting Celsius to Fahrenheit...");
        println!("{temp}°C is equal to {fahrenheit:.2}°F");
    } else {
        println!("Invalid choice, please select 1 or 2");
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
