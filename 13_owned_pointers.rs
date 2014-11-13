fn main() {
  let x = box 10i;

  // let y = x; this doesn't work because x is an owned pointer.
  let y = x.clone();
  println!("{:d}", *x)
  println!("{:d}", *y)
}
