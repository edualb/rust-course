fn main() {
     //let x = 5; storing values in memory (using LIFO)
    // Pusing to stack is faster than allocate to the heap
    // Accessing data on the heap is slower than on the stack
    
    // # Ownership rules
    // * Each value in rust has a variable that's called its owner
    // * There can only be one owner at a time
    // * There can only be one owner at a time
    // let x = 5 (5 is owned by x)

    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    // # Ways variables and Data Interact
    let x = 5; // bind 5 to x
    let y = x; // reate a copy of x and bind to y

    let s1 = String::from("My string"); // Allocating s1 (which constitutes of a pointer, length, and capacity) to the heap
    let s2 = s1; // Setting s2's pointer to equal s1, not a copy! It was moved!

    // println!("{}", s1) you will get an error because it was moved!

    // If you want to keep the value, use s1.clone and the println!("{}", s1) will work.

    // if you try free one of them, you'll get double free error. To avoid it, rust creates: OWNERSHIP!
} // <- x is no longer valid | s is dropped
