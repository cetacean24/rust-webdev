use std::io::Write; // imports write trait; allows stdout().flush() for print!() statement; stdout is linebuffered by default

// could use #[derive(Copy, Clone)]
// instead of copying/moving Conversion, a borrowed reference is passed to the function convert_temperature
enum Conversion {
    FahrenheitToCelsius,
    CelsiusToFahrenheit
}
// enum with two variants: FahrenheitToCelsius, CelsiusToFahrenheit

fn convert_temperature(value: f64, conversion: &Conversion) -> f64 {
    match conversion {
        Conversion::FahrenheitToCelsius => return (value - 32.0) * (5.0/9.0),
        Conversion::CelsiusToFahrenheit => return ((9.0/5.0) * value) + 32.0,
    }
}
// return is not needed here; the last expression in a block is implicitly returned
// based on Conversion, the value has math applied to it

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
// function takes a string slice and returns an f64
// an infinite loop keeps asking the user for input until a valid number is pressed
// flush is required after print!() so that prompt appears before waiting for input
// mutable string required for reading user input into it
// input is trimmed and parsed as a float64 (parse::<f64>()); if sucessful, will return it -> otherwise, loop again + error message
// ::<f64> explicitly tells compiler which type to convert string to; aka, turbofish syntax

fn read_conversion_choice() -> Conversion {
    loop {
        println!("\nEnter 1 or 2 to perform the proper conversion:");
        println!("1.) Fahrenheit to Celsius");
        println!("2.) Celsius to Fahrenheit");

        print!("Your choice: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Conversion::FahrenheitToCelsius,
            "2" => return Conversion::CelsiusToFahrenheit,
            _ => println!("Invalid input. Please enter 1 or 2!"),
        }
    }
}
// user is prompted to select 1 or 2; after print!() input, stdout is flushed
// input is read via stdin().read_line and trims it
// if input matches string "1" or "2", return their respective variant
// if anything but 1 or 2, loop again + error message

fn main() {
    let value = read_number("Enter the temperature you want to convert: ");
    let conversion = read_conversion_choice();

    let result = convert_temperature(value, &conversion);

    match conversion {
        Conversion::FahrenheitToCelsius => {
            println!("\n{:.2}°F is {:.2}°C", value, result);
        }
        Conversion::CelsiusToFahrenheit => {
            println!("\n{:.2}°C is {:.2}°F", value, result);
        }
    }
}
// assigns input from read_number to value
// assigns input from read_conversion_choice to conversion
// calculates result from convert_temperature(value, &conversion)
// based on what conversion was selected in read_conversion_choice, result and value are formatted/printed differently

// IMPORTANT:
// mut allows mutation by the owner
// &mut allows others to mutate via borrowing