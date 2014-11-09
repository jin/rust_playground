fn main() {
  fizzbuzz(1, 100);
}

fn fizzbuzz(min: int, max: int)  {
  for num in range(min, max) {
    let answer: String = 
      if num % 15 == 0 { "FizzBuzz".to_string() } 
      else if num % 3 == 0 { "Fizz".to_string() } 
      else if num % 5 == 0 { "Buzz".to_string() } 
      else { num.to_string() } ;
    
    println!("{}", answer);
  }
}
