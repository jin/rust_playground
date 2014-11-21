struct Rectangle {
  width: int,
  height: int
}

impl Rectangle {
  fn new(width: int, height: int) -> Rectangle {
    Rectangle { width:width, height:height }
  }
}
