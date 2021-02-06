fn main() {
    // # Rule 1
        let mut s1 = String::from("ok1");
        
        // let _r1 = &mut s1;
        // let _r2 = &mut s1;

        // It's got an error, due _r1 created first of _r2 and you are using after declare _r2.
        // println!("{} {}", _r1, _r2);
    // # Rule 2
        // let _r1 = &s1;
        // let _r2 = &mut s1;
        // println!("{} {}", _r1, _r2);

    // # Dangling References
    // let r = dangle(); You'll got an error, due you are missing the reference 
}

// fn dangle() -> &String {
//     let s = String::from("OK");
//     &s;
// }
