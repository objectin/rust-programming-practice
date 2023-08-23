// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: i32,
    weight: f64,
    color: String
}

impl Box {
    fn new(dimensions: i32, weight: f64, color: String) -> Box {
        Box {
            dimensions,
            weight,
            color
        }
    }
    fn display(box: &Box) {
        println!("Dimensions: {}, Weight: {}, Color: {}", box.dimensions, box.weight, box.color);
    }
}

fn main() {
    let shipping_box = Box {
        dimensions: 3, 
        weight: 5.1, 
        color: "Yellow".to_owned()
    };

    Box::display(shipping_box;
    let box2 = Box::new(4, 10.3, String::from("White"));
}
