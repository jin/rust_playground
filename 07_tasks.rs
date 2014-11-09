fn main() {
  for _ in range(0u, 10000) {
    spawn(proc() {
      println!("Hello world");
    });
  }
}
