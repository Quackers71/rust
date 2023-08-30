use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {

    let mut fahrenheit: f64;

    loop {
        eprint!("Enter the temperature in Fahrenheit: ");

        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        fahrenheit = match input.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error parsing input: {}", e);
                continue;
            }
        };

        // Check if the input is within the range of -40 to 100.
        if fahrenheit < -40.0 || fahrenheit > 100.0 {
            println!("The input temperature is out of range. (-40 to 100 degrees Farenheit)");
            continue;
        } else {
            // Limit the output to two decimal places.
            let formatted_celsius = format!("{:.2}", fahrenheit_to_celsius(fahrenheit));
            println!("{} degrees Fahrenheit is equal to {} degrees Celsius", fahrenheit, formatted_celsius);
            break;
        }
    }
}
