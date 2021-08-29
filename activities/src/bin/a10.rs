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

fn display_num_size(size: bool) {
  match size {
    true => println!("It's big"),
    false => println!("It's small"),
  }
}

fn main() {
  let num = 150;
  let size = num > 100;

  display_num_size(size);
}
