fn main() {
    // Variables
    let x = 5;
    println!("immutable x:{}", x);
    // Immutable
    // x = 6 -> cannot do that

    // mutable
    let mut x1 = 5;
    println!("mutable x1 before:{}", x1);
    x1 = 6;
    println!("mutable x1 after:{}", x1);

    // shadowing -> Can change the immutable value
    let x = 6;
    println!("immutable shadowed x:{}", x);

    //constants
    const A_CONSTANT : i32 = 10000;
    println!("const A_CONSTANT:{}", A_CONSTANT);
}
