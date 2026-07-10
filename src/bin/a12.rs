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

// * Use an enum for the box color
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
// * Must include dimensions, weight, and color
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let color = Color::Brown;
    let small_dimensions = Dimensions::new(1.0, 2.0, 3.0);
    let small_box = ShippingBox::new(5.0, color, small_dimensions);

    small_box.print();
}
