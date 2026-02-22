mod search_insert;
mod plus_one;

use crate::search_insert::search_insert_35::search_insert_35;
use crate::plus_one::plus_one;

fn main() {
    let nums = vec![9,8,7,6,5,4,3,2,1,0];
    // println!("search insert: {}", search_insert_35(&nums, 6));

    println!("plus one: {:?}", plus_one(nums));
}