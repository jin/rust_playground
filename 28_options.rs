fn divide(divident: int , divisor: int) -> Option<int> {
    if divisor == 0 {
        None
    } else {
        Some(divident / divisor)
    }
}

fn main() {
    println!("{}", divide(2, 2).unwrap()); // 1
    println!("{}", divide(2, 2));          // Some(1)
    println!("{}", divide(2, 0));          // None
    println!("{}", divide(2, 0).unwrap()); // Panic at unwrapping None
}
