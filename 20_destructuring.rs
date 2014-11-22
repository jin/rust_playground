use std::num::Float;

fn main() {
    // Identical to ruby, but it uses tuples here.
    // a, b = [1, 2]
    let (x, y, z) = ("1", 2i, vec![1u, 2, 3]);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    let a = 2i;
    let b = 3i;

    println!("a: {}, b: {}", a, b);
    let (a, b) = (b, a); // The variable do not need to be mutable
    println!("Swapped! a: {}, b: {}", a, b);

    let (val, (pos_root, neg_root)) = (16u, roots_of(16u));
    println!("Roots of {}: {} and {}", val, pos_root, neg_root);
}

fn roots_of(x: uint) -> (int, int) {
    // non-floats have no access to sqrt wtf?
    ((x as f64).sqrt() as int, -((x as f64).sqrt()) as int)
}
