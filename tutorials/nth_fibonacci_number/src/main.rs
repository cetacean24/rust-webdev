use std::io::Write;

fn calculate_fibonacci_number(nth: i32) -> i32 {
    // first added to second -> first becomes second -> second becomes new total (temp) -> repeat pattern nth amount of times
    // cases in which fibonacci number is easily known
    if nth == 0 {
        return 0;
    }
    if nth == 1 {
        return 1;
    }

    // fibonacci sequence example -> top = value; bottom = function F()
    // example: 1 + 1 = 2 | 2 + 1 = 3 | 3 + 2 = 5
    // number   2   1   3 | 3   2   4 | 4   3   5

    // start the loop on the nth = 2 fibonacci number
    let mut first: i32 = 0; // F(0)
    let mut second: i32 = 1; // F(1)
    // F(2) = 0 + 1; F(2) = 1; if nth = 2 that would be the output


    for _ in 2..=nth { // loop starts from two and includes nth; already know nth = 0 and nth = 1
        let temp: i32 = first + second; // temp holds sum of first + second numbers (next fibbonacci number)
        first = second; // first number becomes second number for next iteration
        second = temp; // second becomes the old fibonacci number; if at end of loop, return it, else, add the next number to it
    }
    return second // second holds the nth fibonacci number after loop finishes
}

fn main() {

    loop {
        print!("What nth Fibonacci number would you like to generate: ");
        std::io::stdout().flush().unwrap();
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    
        match input.trim().parse::<i32>() {
            Ok(num) => {
                let result: i32 = calculate_fibonacci_number(num);
                println!("The Fibonacci number of {} is {}", num, result);
                break
            },
            Err(_) => println!("Please enter a valid i32 number"),
        }
    }
}
