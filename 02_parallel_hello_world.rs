use std::thread::spawn;

fn main() {
  for _ in (0u32..10) {
    spawn(move || {
      let message = "Hello?";
      println!("{}", message);
    });
  }
}
