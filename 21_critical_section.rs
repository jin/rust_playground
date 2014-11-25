fn main() {
    let mut c1 = false;
    let mut c2 = false;

    spawn(proc() {
        while true {
           c1 = true;
           while c2 {}
           if c2 { println!("Mutual exclusion broken") } // This never gets printed
           critical_section("c1".to_string());
           c1 = false;
        }
    });

    spawn(proc() {
        while true {
           c2 = true;
           while c1 {}
           if c1 { println!("Mutual exclusion broken") } // This never gets printed
           critical_section("c2".to_string());
           c2 = false;
        }
    });
}

fn critical_section(caller: String) {
    println!("Called by {}", caller);
}
