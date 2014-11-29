// Example from Steve Klabnik's All Things Open 2014 talk on Rust.
//
// → ./27_arc_mutex 
// numbers[0] is 1
// numbers[1] is 3
// numbers[2] is 5

use std::sync::Mutex;
use std::sync::Arc;

pub fn main() {
    // Arc: Atomic reference counting
    // This prevents a dangling pointer when a thread dies with this vec
    let numbers = Arc::new(Mutex::new(vec![1i, 2i, 3i]));

    for i in range(0u, 3u) {
        // .clone() increases the refcount by 1
        let number = numbers.clone();

        // Create closure
        spawn(proc() {
           let mut array = number.lock(); 
           (*array)[i] += i as int;
           println!("numbers[{}] is {}", i, (*array)[i]);

           // The lock is automically released at compile time 
           // when the array goes out of scope.
           // The reference count is also decreased by 1 automatically.
        });
    }
}
