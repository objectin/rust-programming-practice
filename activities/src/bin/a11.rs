// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id_number: i32
}

fn disp_quantity(grocery: &Grocery) {
    println!("quantity: {}", grocery.quantity);
}

fn disp_id(grocery: &Grocery) {
    println!("id: {}", grocery.id_number);
}

fn main() {
    let brokoli = Grocery {
        quantity: 10,
        id_number: 1234
    };

    disp_quantity(&brokoli);
    disp_id(&brokoli);
}
