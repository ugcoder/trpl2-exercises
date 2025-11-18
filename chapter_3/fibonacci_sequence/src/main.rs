use std::io;

fn fibonacci(n: usize) -> Vec<u128> {
    if n == 0 {
        return vec![];
    };
    if n == 1 {
        return vec![0];
    }
    let mut vec = vec![0u128; n];
    vec[1] = 1;
    for i in 2..n {
        vec[i] = vec[i - 1] + vec[i - 2];
    }
    vec
}

fn main() {
    println!("Welcome to Fibonacci sequence generator!\n(Enter '^c' or 'q' to exit)");
    eprint!(": ");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "^c" || input.trim() == "q" {
            println!("Bye Bye!");
            break;
        }

        let input = input.trim().parse::<usize>();

        let n = match input {
            Ok(num) if num > 0 => num,
            _ => {
                println!("Not an integer ");
                continue;
            }
        };

        let fib = fibonacci(n);
        println!("{:?}", fib);
        eprint!(": ")
    }
}
