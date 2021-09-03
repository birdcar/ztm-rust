// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Customer {
  name: String,
  age: u8,
}

fn card_customer(customer: &Customer) -> Result<(), String> {
  if customer.age < 21 {
    Err("Customer must be at least 21 years old".to_owned())
  } else {
    Ok(())
  }
}

fn main() {
  let katie = Customer {
    name: String::from("Katie"),
    age: 32,
  };
  let katie_purchase = card_customer(&katie);
  println!("{:?}", katie_purchase);

  let jay = Customer {
    name: String::from("Jay"),
    age: 10,
  };
  let jay_purchase = card_customer(&jay);
  println!("{:?}", jay_purchase);
}
