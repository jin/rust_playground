fn main() {
  let (chan, port) = channel();

  spawn(proc() {
    chan.send("42u");
  });

  println!("{:s}", port.recv().to_string());
}
