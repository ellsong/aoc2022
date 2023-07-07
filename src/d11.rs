use crate::d11b;
use crate::d11a;

pub fn d11(part: i32) {
    match part {
        1 => d11a::p1(),
        2 => d11b::p2(),
        _ => println!("Invalid part"),
    }
}
