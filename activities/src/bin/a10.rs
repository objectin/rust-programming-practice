// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_msg(state: bool) {
    if state {
        println!("its big");
    } else {
        println!("its small");
    }
}

fn main() {
    let n = 100;
    let state = if n > 100 {
        true
    } else {
        false
    };
    print_msg(state);
    print_msg(state);
}
