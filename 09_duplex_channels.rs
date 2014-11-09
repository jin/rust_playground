use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
  let mut value: int;

  loop {
    value = receiver.recv();
    sender.send(value + 1);
    if value == 0 { break; };
  }
}

fn main() {
  let (from_parent_sender, from_parent_receiver) = channel();
  let (from_child_sender, from_child_receiver) = channel();

  spawn(proc() {
    plus_one(&from_child_sender, &from_parent_receiver);
  });

  from_parent_sender.send(42i);
  from_parent_sender.send(43i);
  from_parent_sender.send(44i);
  from_parent_sender.send(45i);

  // Kill child proc when program ends.
  from_parent_sender.send(0);

  for _ in range(0u, 4) {
    let answer = from_child_receiver.recv();
    println!("{}", answer.to_string());
  }
}
