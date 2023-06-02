use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input) // read_line returns a Result type
            .expect("Failed to read line"); // expect is a method of Result type
                                        
        // shadowing the input variable
        // Which lets us convert the type of the variable from a string to an unsigned 32-bit integer
        // We do this because the cmp method only works on the same types
        // We also use the match expression to handle the error case
        // The parse method returns a Result type as well
        // We use the Ok variant to return the number
        // We use the Err variant to continue the loop and ask the user to enter a number again
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {}", input);

        // cmp method compares two values and can be called on anything that can be compared
        // Ordering has three variants: Less, Greater, and Equal
        // We use a match expression to handle each variants
        // We use the cmp method to compare the input and secret_number
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // break out of the loop
                println!("You win!");
                break;
            },
        }
    }
}
