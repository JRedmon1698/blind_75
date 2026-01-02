fn main() {
    println!("{}", search(vec![-1, 3, 6, 9, 11], 9i32));
    println!("{}", search(vec![-1, 3, 6, 9, 11], 2i32));
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;

    while low <= high {
        let mut mid: usize = low + (high - low / 2);

        if target == nums[mid] {
            return mid as i32;
        } else if target < nums[mid] {
            // prevent underflow
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    -1
}
