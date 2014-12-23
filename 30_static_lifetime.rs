// From rustbyexample
//
// String literals like "string" can also be assigned to static variables. 
// These variables have type signature &'static str, and are references to strings allocated in read-only memory. 
// 'static is a special lifetime that outlives all the other lifetimes, and indicates that the referenced data is available in all the scopes.


fn get_string() -> &'static str {
    "str"
}

fn main() {
    println!("{}", get_string());
}
