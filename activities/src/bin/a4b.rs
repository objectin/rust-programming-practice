// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

use std::io;

fn main() {
    // Get int input
    let mut input = String::new();
    print!("Enter an integer: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");
    let output = match num {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    println!("{}", output);
}
