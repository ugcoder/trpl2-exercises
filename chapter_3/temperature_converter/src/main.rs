use std::io;

fn f_to_c() {
    loop {
        eprint!("Please enter Fahrenheit: ");

        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse Fahrenheit");
                continue;
            }
        };

        println!();
        println!(
            "{}° Fahrenheit is {}°C.",
            fahrenheit,
            (fahrenheit - 32.0) * 5.0 / 9.0
        );
        println!();
        break;
    }
}

fn c_to_f() {
    loop {
        eprint!("Please enter celsius: ");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse celsius");
                continue;
            }
        };

        println!();
        println!(
            "{}° Celsius is {}°F.",
            celsius,
            (celsius * 9.0 / 5.0) + 32.0
        );
        println!();
        break;
    }
}

fn main() {
    println!("========================");
    println!("Temperature Converter");
    println!("========================");

    loop {
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("q: quit");
        println!("========================");

        let mut choice = String::new();
        eprint!("Enter your choice: ");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Celsius to Fahrenheit");
                c_to_f();
                continue;
            }
            "2" => {
                println!("Fahrenheit to Celsius");
                f_to_c();
                continue;
            }
            "q" => break println!("Bye Bye!"),
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            }
        };
    }
}
