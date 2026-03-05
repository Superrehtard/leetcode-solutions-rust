mod search_insert;
mod plus_one;
mod p58;
mod p209;

use crate::plus_one::plus_one;
use p58::length_of_last_word;
use p209::min_sub_array_len;

fn main() {
    let nums = vec![9,8,7,6,5,4,3,2,1,0];
    // println!("search insert: {}", search_insert_35(&nums, 6));

    println!("plus one: {:?}", plus_one(nums));

    length_of_last_word("hello world");

    let mut iter = "".split_ascii_whitespace();
    println!("{:?}", iter.next());

    let num_array = vec![1,2,3,4,5];

    println!("{}", min_sub_array_len(11, num_array));
}