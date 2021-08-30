// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
  name: String,
  locker_num: Option<i32>,
}

fn main() {
  let students = vec![
    Student {
      name: "Jay".to_owned(),
      locker_num: None,
    },
    Student {
      name: "Enrique".to_owned(),
      locker_num: Some(34),
    },
  ];

  for student in &students {
    match student.locker_num {
      Some(locker_num) => println!("{} is assigned Locker #{}", student.name, locker_num),
      None => println!("{} does not have a locker assigned", student.name),
    }
  }
}
