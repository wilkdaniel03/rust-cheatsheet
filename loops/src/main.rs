fn main() {
    // Loops in rust
    // while_loop();
    // while_let_loop();
    // loop_loop();
    for_loop();
}

#[allow(dead_code)]
fn while_loop() {
    // I think, while is the simplest one 
    let mut x = 0;
    while x < 10 {
        println!("{}", x);
        x += 1;
    }
}

#[allow(dead_code)]
fn while_let_loop() {
    // There is also somethink called while let loop
    // this loop will iterate as long as
    // nums.pop() won't return any error
    let mut nums = vec![1, 2, 3];
    while let Some(x) = nums.pop() {
        println!("{}", x);
    }
}

#[allow(dead_code)]
fn loop_loop() {
    // A loop loop will iterate forever until
    // somethink eventually will break the loop
    let mut x = 0;
    loop {
        match x {
            0..=4 => {
                println!("{}", x);
                x += 1;
            },
            _ => break,
        }
    }
}

fn for_loop() {
    // Old good for loops
    // python-range-like syntax
    for x in 1..11 {
        println!("{}", x);
    }
}
