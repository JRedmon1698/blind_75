use std::collections::HashMap;

/*
Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
*/

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<&i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let addend = target - *num;
        if map.contains_key(&addend) {
            return vec![*map.get(&addend).unwrap() as i32, i as i32];
        } else {
            map.insert(num, i);
        }
    }

    return Vec::new();
}
