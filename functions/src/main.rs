fn main() {
    // # Functions
    let mut  x : u8 = 1;
    println!("Before the function: {}", x);
    x = stuff(x);
    println!("After the function: {}", x);

    // ## Satatements
    // - ';'
    // - Dont return value

    // ## Expressions
    // - no ';' Or have 'return' keyword
    // - return value
}

fn stuff(x : u8) -> u8 {
    println!("Inside of function: {}", x);
    return x + 1
}
