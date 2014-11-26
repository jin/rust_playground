fn main() {
    let (p1_send, p2_recv): (Sender<uint>, Receiver<uint>) = channel();
    let (p2_send, p1_recv): (Sender<uint>, Receiver<uint>) = channel();

    let mut x = 0u;
    let p1 = proc() {
        loop {
            p1_send.send(x);
            x = p1_recv.recv() + 1;
            println!("P1 received {}", x);
        }
    };

    let p2 = proc() {
        loop {
            let y = p2_recv.recv() + 1;
            println!("P2 received {}", y);
            p2_send.send(y);
        }
    };

    spawn(p1);
    spawn(p2);
}
