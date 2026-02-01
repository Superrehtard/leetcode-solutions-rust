fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, numbers.len().saturating_sub(1));

    while left < right {
        if numbers[left] + numbers[right] > target {
            right -= 1;
        } else if (numbers[left] + numbers[right] < target) {
            left += 1;
        } else {
            return vec![left as i32 + 1, right as i32 + 1];
        }
    }

    vec![left as i32 + 1, right as i32 + 1]
}