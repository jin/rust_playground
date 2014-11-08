fn main() {
  for _ in range(0u, 10) {
    spawn(proc() {
      let message = "Hello?";
      println!("{}", message);
    });
  }
}
