use std::collections::HashMap;

fn main() {
    // CREATING AD ADDING
    
    let mut hm : HashMap<String, i32> = HashMap::new();
    hm.insert("blue".to_string(), 10);
    hm.insert("red".to_string(), 100);
    println!("{:?}", hm);
    
    // ACCESSING
    // // Method #1 (GET):
    println!("{:?}", hm.get(&"blue".to_string()));

    // // Method #2 ([]):
    println!("{:?}", &hm[&"blue".to_string()]);

    for (key, value) in &hm {
        println!("{}, {}", key, value);
    }

    // UPDATING

    // // Overwriting

    hm.insert("blue".to_string(), 5);
    println!("{:?}", hm);// change "blue" value

    // // Create iff !existent

    hm.entry("red".to_string()).or_insert(10000);
    println!("{:?}", hm); // does not change "red" value
    hm.entry("yellow".to_string()).or_insert(10000);
    println!("{:?}", hm); // inserts yellow : 10000 value

    // // derefs

    let mut hm2 = HashMap::new();
    hm2.insert("k1", 1);
    hm2.insert("k2", 2);
    println!("{:?}", hm2);

    for (_, val) in &mut hm2 {
        *val += 1;
    }
    println!("{:?}", hm2);

}
