pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted = nums.clone();
    sorted.sort();
    let mut result = Vec::<Vec<i32>>::new();

    for i in 0..sorted.len().saturating_sub(2) {
        if sorted[i] > 0 {
            break;
        }
        if i > 0 && sorted[i] == sorted[i - 1] {
            continue;
        }
        let mut sliced_array = sorted[i + 1..].to_vec();

        let two_sum_result = two_sum(sliced_array, -(sorted[i]));
        for j in 0..two_sum_result.len() {
            result.push(vec![sorted[i], two_sum_result[j][0], two_sum_result[j][1]]);
        }
    }

    result
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let (mut left, mut right) = (0, numbers.len().saturating_sub(1));
    let mut result = Vec::<Vec<i32>>::new();

    while left < right {
        if numbers[left] + numbers[right] > target {
            right -= 1;
        } else if (numbers[left] + numbers[right] < target) {
            left += 1;
        } else {
            result.push(vec![numbers[left], numbers[right]]);
            right -= 1;
            left += 1;
            while left < right && numbers[right] == numbers[right - 1] {
                right -= 1;
            }
            while left < right && numbers[left] == numbers[left + 1] {
                left += 1;
            }
        }
    }

    result
}