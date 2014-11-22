// Rust has no C-style loops
// Iterators uses generics and lazy (no intermediate arrays)

fn main() {
    // Chained method calls!
    for num in range(0u, 100u).filter( |&num| num % 6 == 0 ).map( |num| num * 3 ) {
        println!("{}", num);
    }
}
