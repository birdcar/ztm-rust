// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum Colors {
  Red,
  Orange,
  Yellow,
  Green,
  Blue,
  Indigo,
  Violet,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Colors) {
  // * Use a match expression to determine which color name to print
  match color {
    Colors::Red => println!("Red"),
    Colors::Orange => println!("Orange"),
    Colors::Yellow => println!("Yellow"),
    Colors::Green => println!("Green"),
    Colors::Blue => println!("Blue"),
    Colors::Indigo => println!("Indigo"),
    Colors::Violet => println!("Violet"),
  }
}

fn main() {
  print_color(Colors::Red);
}
