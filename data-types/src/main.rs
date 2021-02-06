fn main() {
    // Explicit vs Implicit
    let mut e: u8 = 0;
    let mut i = 0; //
    // # Scalar Types
    // ## Integers
    // ### Unsigned (i.e. u8) VS Signed (i.e. i8)
    let mut int : u8 = 255;
    // ## Floats
    let mut float : f32 = 2.0; // f64 default
    // ## Bool
    let mut b : bool = true;
    // ## Char
    // ### ASCII VS Unicode Scalar Value
    // 1 byte VS. 4 bytes
    let mut c = ':';
    println!("{}", c);

    // # Compound Types
    // ## Tuples
    let mut tup = (1, 2, 'c');
    println!("{:?}", tup);
    tup.2 = 'a';
    println!("{:?}", tup);
    let (x, y, z) = tup;
    println!("{}", x);
    let mut tup1 = (tup, 1);
    println!("{:?}", tup1);
    println!("{}", (tup1.0).2);

    // ## Arrays
    let mut arr = [1,2,3];
    let mut arr1 : [u8;3] = [1,2,3];
    println!("{:?}", arr1);
    arr1[0] = 20;
    println!("{:?}", arr1);

    // # Curiosities
    // ## Two's complement
    // ## Overflow
    

}
