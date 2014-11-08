fn divisible_by_three(num: int) -> bool {
  num % 3 == 0
}

fn divisible_by_five(num: int) -> bool {
  num % 5 == 0
}

fn divisible_by_fifteen(num: int) -> bool {
  num % 15 == 0
}

fn main() {
  for num in range(1i, 100) {
    let answer: &str = if divisible_by_fifteen(num) {
      "FizzBuzz"
    } else if divisible_by_five(num) {
      "Buzz"
    } else if divisible_by_three(num) {
      "Fizz"
    } else {
      ""
    };

    println!("{}", answer)
  }
}

#[test]
fn test_one_divisible_by_three() {
  if divisible_by_three(1) {
    fail!("1 is not divisible by 3.");
  }
}

#[test]
fn test_three_divisible_by_three() {
  if !divisible_by_three(3) {
    fail!("3 is divisible by 3.");
  }
}

#[test]
fn test_one_divisible_by_five() {
  if divisible_by_five(1) {
    fail!("1 is not divisible by 5.");
  }
}

#[test]
fn test_five_divisible_by_five() {
  if !divisible_by_five(5) {
    fail!("5 is divisible by 5.");
  }
}

#[test]
fn test_one_divisible_by_fifteen() {
  if divisible_by_fifteen(1) {
    fail!("1 is not divisible by 15.");
  }
}

#[test]
fn test_fifteen_divisible_by_fifteen() {
  if !divisible_by_three(15) {
    fail!("15 is divisible by 15.");
  }
}
