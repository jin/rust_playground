fn main() {
  let names = vec!("foo", "bar", "baz");
  let numbers = vec!(1i, 2, 3);
  let more_numbers = vec!(4i, 5, 6);

  let all_numbers = numbers + more_numbers; 

  for number in all_numbers.iter() {
    println!("{}", number);
  }

  let name = names.get(2);

  // This prints "Some(baz)" because it is an existing Option type
  println!("{}", name);

  // This dereferences the pointer and prints "baz"
  match name {
    Some(name) =>{ println!("{}", *name) },
    None => {}
  }

  // This does not raise an error because 
  // non-existent values in a vector are treated as None, 
  // an Option type.
  let out_of_bounds_name = names.get(3);
  // This prints "None" because it doesn't exist.
  println!("{}", out_of_bounds_name);
}
