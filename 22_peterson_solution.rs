use std::io::Timer;
use std::time::Duration;

fn main() {
    let mut c1 = false;
    let mut c2 = false;

    // This can only contain the id of one proc at a time
    // Last to set this will be forced to wait.
    let mut will_wait = 1u; 

    spawn(proc() {
        while true {
           c1 = true;
           will_wait = 1u;
           while c2 && will_wait == 1 {}
           critical_section("c1".to_string());
           c1 = false;
        }
    });

    spawn(proc() {
        while true {
           c2 = true;
           will_wait = 2u;
           while c1 && will_wait == 2 {}
           critical_section("c2".to_string());
           c2 = false;
        }
    });
}

fn critical_section(caller: String) {
    let mut timer = Timer::new().unwrap();
    println!("Called by {}, timing out for 300ms..", caller);
    timer.sleep(Duration::milliseconds(300));
}
