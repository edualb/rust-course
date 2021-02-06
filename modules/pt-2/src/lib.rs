// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist(){}
//     }
// }

mod front_of_house; // src/front_of_house.rs

use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}














// use std::io;
// use std::cmp::Ordering;
// use std::{cmp::Ordering, io};

// use std::io::Write;
// use std::io;
// use std::{self, Write};

// use std::collections::*;














// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist(){
//             hosting_chaos::hosting2::add_to_waitlist();
//         }
//         pub mod hosting_chaos {
//             pub use crate::front_of_house2::hosting2;
//         }
//     }
// }

// mod front_of_house2 {
//     pub mod hosting2 {
//         pub fn add_to_waitlist(){}
//     }
// }

// use front_of_house::hosting::add_to_waitlist;
// use front_of_house2::hosting2::add_to_waitlist as add_to_waitlist2;

// fn eat_at_restaurant() {

//     add_to_waitlist();
//     add_to_waitlist2();

// }