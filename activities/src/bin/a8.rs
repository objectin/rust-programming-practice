// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Spicy,
    Sparkling,
    Sweet
}

struct Drink {
    flavor: Flavor,
    ounces: f64
}

fn print_drink(drink: Drink) {
    let output = match drink.flavor {
        Flavor::Spicy => "spicy!",
        Flavor::Sparkling => "sparkling!",
        Flavor::Sweet => "sweet!",
    };

    println!("{} - {} ounces", output, drink.ounces);
}

fn main() {
    let drink = Drink {flavor:Flavor::Sparkling, ounces:6.0};
    print_drink(drink);
}


