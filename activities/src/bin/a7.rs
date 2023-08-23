// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color{
    Red,
    Blue,
    Yellow
}

fn main() {
    let color = Color::Red;
    let output = match color {
        Color::Red => "red",
        Color::Blue => "blue",
        Color::Yellow => "yellow",
    };

    println!("Color name is {}", output);
}
