fn main() {
  let nums = [1u, 2u];
  let names = ["foo", "bar", "baz", "qux"];

  let mut nums = nums.iter().map({|&x| x * 2 - 1});

  for num in nums {
    println!("{}", names[num].to_string());
  }
}
