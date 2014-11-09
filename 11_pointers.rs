fn plus_one(x: &int) -> int {
  x + 1
}

fn plus_one_alt(x: int) -> int {
  x + 1
}

// Cannot dereference immutable pointer
// fn change_me(x: &int) {
//   *x = 42;
// }

fn succ(x: &int) -> int {
  *x + 1
}

fn main() {
  let x = 5i;
  println!("{}", plus_one(&x).to_string());
  println!("{}", plus_one_alt(x).to_string());
  println!("{}", &x.to_string());
  println!("{}", x.to_string());

  let y = 8i;
  let z = &y;

  println!("{:p}", z);
  println!("{}", x + *z);

  println!("{}", succ(&y));
  println!("{}", succ(z));

  // Mutable pointers
  
  let mut x = 42i;
  let y = &mut x;

  println!("{}", y);
  println!("{:p}", &y);
  // But you cannot mutate borrowed variables
  // x -= 1;
  // Or alias a mutable pointer more than once
  // let k = &mut x;
}
