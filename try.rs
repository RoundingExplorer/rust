use std::io
fn main() {
  println!("Enter a number: ")
  let mut number = String::new();
 
  io::stdin().read_line(&mut number).expect("failed to readline");
  println!("You entered: ", number)
}
