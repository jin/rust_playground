use std::rand;
fn main() { 
    let max = 42i; 
    println!("{}" , get_random(&max)); 
}

fn get_random(max: &int) -> int {
    (rand::random::<u32>() as int) % (*max + 1)
}
