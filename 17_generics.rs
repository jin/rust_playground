// Traits and generics

fn main() {
  let vec = [1i, 2, 3];
  print_int_vec(&vec);

  let str_vec = ["foo", "bar", "baz"];
  print_str_vec(&str_vec);


  print_vec(&vec);
  print_vec(&str_vec);
}

fn print_int_vec(vec: &[int]) {
    for elem in vec.iter() {
        println!("{}", *elem);
    }
}

fn print_str_vec(vec: &[&str]) {
    for elem in vec.iter() { 
        println!("{}", *elem);
    }
}

// <T> -> Make function polymorphic over type T
fn print_vec<T: std::fmt::Show>(vec: &[T]) {
    for elem in vec.iter() {
        println!("{}", *elem)
    }
}
