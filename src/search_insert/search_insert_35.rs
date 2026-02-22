pub fn search_insert_35(nums: Vec<i32>, target: i32) -> i32 {
    if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    }

    if target <= nums[0] {
        return 0;
    }

    let mut start: usize = 0;
    let mut end: usize = nums.len();
    println!("Start: {} and End: {}", start, end);

    while start < end {
        let mid = start + (end - start) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start as i32
}