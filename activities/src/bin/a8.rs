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
  Lime,
  Lemon,
  PricklyPear,
}

struct Drink {
  flavor: Flavor,
  fluid_oz: f64,
}

fn print_drink_data(drink: Drink) {
  let flavor = match drink.flavor {
    Flavor::Lime => String::from("Lime"),
    Flavor::Lemon => String::from("Lemon"),
    Flavor::PricklyPear => String::from("Prickly Pear"),
  };

  println!("{:?} ({:?} oz)", flavor, drink.fluid_oz);
}

fn main() {
  let prickly_pear_pint = Drink {
    flavor: Flavor::PricklyPear,
    fluid_oz: 16.0,
  };

  print_drink_data(prickly_pear_pint);
}
