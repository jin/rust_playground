// ./29_nested_named_loops
// Start
// Enter level 1
// Enter level 2a
// Enter level 3
// Enter level 2b
// End

fn main() {
    println!("Start");
    'level1: loop {
        println!("Enter level 1");
        'level2a: loop {
            println!("Enter level 2a");
            'level3: loop {
                println!("Enter level 3");
                break 'level2a;
                println!("Exit level 3");
            }
            println!("Exit level 2a");
        }
        'level2b: loop {
            println!("Enter level 2b");
            break 'level1;
            println!("Exit level 2b");
        }
        println!("Exit level 1");
    }
    println!("End");
}
