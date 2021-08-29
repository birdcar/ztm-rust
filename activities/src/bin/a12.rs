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

enum Color {
  Branded,
}

impl Color {
  fn print(&self) {
    match self {
      Color::Branded => println!("Color: Branded"),
    }
  }
}

struct Dimensions {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimensions {
  fn print(&self) {
    println!(
      "Dimensions:\n\tLength: {:?}\n\tWidth: {:?}\n\tHeight: {:?}",
      self.depth, self.width, self.height
    );
  }
}

struct Box {
  weight: f64,
  dimensions: Dimensions,
  color: Color,
}

impl Box {
  // Generates a new Box
  fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
    Self {
      dimensions,
      weight,
      color,
    }
  }

  // Prints a box's label, including the required info for shipping
  fn print(&self) {
    self.color.print();
    println!("Weight: {:?}", self.weight);
    self.dimensions.print();
  }
}

fn main() {
  let std_size = Dimensions {
    width: 5.0,
    height: 5.0,
    depth: 5.0,
  };

  let branded_box = Box::new(std_size, 15.0, Color::Branded);

  branded_box.print();
}
