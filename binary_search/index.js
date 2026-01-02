/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function(nums, target) {
    let high = nums.length - 1;
    let low = 0;

    while (low <= high) {
      let mid = low + Math.floor((high - low) / 2);

      if (target === nums[mid]) {
        return mid;
      } else if (target > nums[mid]) {
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }

    return -1;
};

console.log(search([-1,0,3,5,9,12], 2));
