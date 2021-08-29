// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coord() -> (i32, i32) {
  (2, 9)
}

fn main() {
  let (x, y) = get_coord();

  if y < 5 {
    println!("({:?}, {:?})'s y is less than 5", x, y)
  } else if y == 5 {
    println!("({:?}, {:?})'s y is 5", x, y);
  } else {
    println!("({:?}, {:?})'s y is greater than 5", x, y);
  }
}
