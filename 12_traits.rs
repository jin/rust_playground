trait Printable {
  fn print_explosivity(&self);
  fn print_kind(&self);
}

struct TimeBomb {
  explositivity: uint,
  kind: String
}

impl Printable for TimeBomb {
  fn print_explosivity(&self) { println!("{}", self.explositivity); }
  fn print_kind(&self) { println!("{}", self.kind); }
}

fn main() {
  let x = TimeBomb { explositivity: 5, kind: "electronic".to_string() };
  x.print_explosivity();
  x.print_kind();
}
