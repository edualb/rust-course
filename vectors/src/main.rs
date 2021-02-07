fn main() {
    // Vectors
    let mut v : Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];

    // ADD
    v.push(5);
    v.push(1);
    println!("{:?}", &v);

    // REMOVE
    v.pop(); // LIFO
    println!("{:?}", &v);

    v.push(5);
    v.push(1);
    v.push(5);
    v.push(1);
    println!("{:?}", &v);
    v.remove(0); // remove the index 0
    println!("{:?}", &v);

    // GET
    println!("{:}", &v[0]); // returns 5
    println!("{:?}", v.get(0)); // returns Some(5)

    // println!("{:}", &v[100]); // panic!!
    println!("{:?}", v.get(100)); // returns None

    // Be Careful!
    let r = &v1[0]; // here is immutable
    v1.push(6); // here is changing
    // println!("{:?}", r) // we cannot have a mutable borrow after an immutable borrow

    // ITERATING

    for i in &v {
        println!("{:?}", i);
    }

    for i in 0..v.len() {
        println!("{:?}", i);
    }

    // Vectors of different types?
    #[derive(Debug)]
    enum TYPE {
        INTEGER(i32),
        STRING(String),
    }

    let mut v2 = Vec::new();
    v2.push(TYPE::INTEGER(3));
    v2.push(TYPE::STRING(String::from("ok")));

    println!("{:?}", v2);

    match &v2[0] {
        TYPE::INTEGER(i) => {
            println!("{}", i);
        },
        TYPE::STRING(s) => {
            println!("{}", s);
        }
    }

    // derefs

    let mut v9 = vec![1, 2, 3];
    println!("{:?}", v9);

    for val in &mut v9 {
        *val += 1;
    }
    
    println!("{:?}", v9);
}