pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    // better way to do this is in-place
    // same logic, we iterate from the end
    // and we stop as soon as there is no carry_over

    // iterating over a vector in reverse, good to remember
    // for i in (0..digits.len()).rev() {
    //     println!("{}", digits[i]);
    // }
    let mut reverse_digits = digits.clone();
    reverse_digits.reverse();
    let mut carry_over = 1;
    let mut result = Vec::<i32>::new();

    for mut item in reverse_digits {
        item += carry_over;
        if item > 9 {
            carry_over = 1;
            result.push(0);
        } else {
            carry_over = 0;
            result.push(item + carry_over);
        }
    }

    if carry_over > 0 {
        result.push(carry_over);
    }
    result.reverse();

    result
}