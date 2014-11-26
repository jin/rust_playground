use std::num::pow;

pub fn main() {

    let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();
    let fib_tx = tx.clone();
    let square_tx = tx.clone();

    let fib = proc() {
        let mut x = 0u64;
        let mut y = 1u64;
        loop {
            let new = x + y;
            x = y;
            y = new;
            fib_tx.send(y);
        }
    };

    let square = proc() {
        let mut x = 1u;
        loop {
            square_tx.send(pow(2u64, x));
            x += 1;
        }
    };

    let printer = proc() {
        loop {
            println!("Received: {}", rx.recv());
        }
    };

    spawn(printer);
    spawn(fib);
    spawn(square);
}

