// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum Department {
  Maintenance,
  Managers,
  Marketing,
  LineSupervisors,
  Kitchen,
  AssemblyTechnicians,
}

#[derive(Debug)]
enum Status {
  Active,
  Terminated,
}

#[derive(Debug)]
struct Employee {
  name: String,
  dept: Department,
  status: Status,
}

fn try_access(emp: &Employee) -> Result<(), String> {
  match emp.status {
    Status::Terminated => return Err("Terminated".to_owned()),
    _ => (),
  }

  match emp.dept {
    Department::Maintenance => Ok(()),
    Department::Managers => Ok(()),
    Department::Marketing => Ok(()),
    _ => Err("Access Denied".to_owned()),
  }
}

fn print_access(emp: &Employee) -> Result<(), String> {
  let attempt_access = try_access(emp)?;
  println!("Access ok!");
  Ok(attempt_access)
}

fn main() {
  let emp = Employee {
    name: String::from("Nick"),
    dept: Department::Managers,
    status: Status::Terminated,
  };

  match print_access(&emp) {
    Err(e) => println!("Error: {:?}", e),
    _ => (),
  }
}
