// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
  age: u8,
  name: String,
  color: String,
}

impl Person {
  fn print(&self) {
    println!("name: {:?}\ncolor: {:?}\n", self.name, self.color)
  }
}

fn main() {
  let people = vec![
    Person {
      age: 10,
      name: String::from("Jay"),
      color: String::from("Green"),
    },
    Person {
      age: 33,
      name: String::from("Nick"),
      color: String::from("Black"),
    },
    Person {
      age: 32,
      name: String::from("Dani"),
      color: String::from("Purple"),
    },
  ];

  for person in &people {
    if person.age < 33 {
      person.print();
    }
  }
}
