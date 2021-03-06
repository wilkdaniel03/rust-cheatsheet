// Rust is a compiled programming language
// what mean's the compiler have to know
// every variable's type at compile time

fn main() {
    scalar_types();
    collection_types();
}

#[allow(unused_variables)]
fn scalar_types() {
    // Scalar types - represent single values
    // Integer types
    let int: i8 = 100; /* -128 - 127 | 0 - 255 */ let int: u8 = 200;
    let int: i16 = 100; /* -32768 - 32767 | 0 - 65535 */ let int: u16 = 200;
    let int: i32 = 100; /* ~-2Bil - ~2Bil | 0 - ~4Bil */ let int: u32 = 200;
    let int: i64 = 100; /* The really big one */ let int: u64 = 200;
    let int: i128 = 100; /* A LOT */ let int: u128 = 200;
    // The think is we certainly should use 32/64 bits depending on system arch
    // for big numbers or just use that
    let int: isize = 100; let int: usize = 200;
    // 128 bits are certainly used in cryptography
    

    // Integer literals
    let num = 10_100;           // Decimal
    let num = 0xff;             // Hex
    let num = 0o77;             // Octal
    let num = 0b1111_0000;      // Binary

    // Floating point types
    let float: f32 = 2.0;
    let float: f64 = 2.0;
}

#[allow(unused_variables)]
fn collection_types() {
    // Collection types - represent multiple values
    // Tupples allow us to group together different types
    let tup: (u8, u16, u32) = (200, 20000, 2000000);
    let (x, y, z) = tup;
    let second_value = tup.1; // -> 20000
    

    // Array type
    // every element in array have to have the same type
    // useful data type when allocating data on stack
    let nums: [i8; 5] = [1, 2, 3, 4, 5];
    let num = nums[2]; // -> 3
}
