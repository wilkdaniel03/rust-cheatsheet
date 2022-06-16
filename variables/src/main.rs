fn main() {
    // immutable_variable();
    mutable_variable();
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

fn mutable_variable() {
    // Declare a mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}
