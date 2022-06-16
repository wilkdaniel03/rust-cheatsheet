fn main() {
    // immutable_variable();
    // mutable_variable();
    constant();
}

#[allow(dead_code)]
fn immutable_variable() {
    // Declare a immutable variable
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);
    // That would return an error because
    // we cannot edit the x variable.
}

#[allow(dead_code)]
fn mutable_variable() {
    // Declare a mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

#[allow(dead_code)]
fn constant() {
    // Declare a constant
    const X: i32 = 5;
    // Constants have to have declared type and hardcoded value
    // Constants should be named in uppercase
}
