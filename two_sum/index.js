/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
// var twoSum = function(nums, target) {
//     let sum = 0;
//     const indexes = [];

//     for (let i = 0; i < nums.length - 1; i++) {
//       for (let k = 1; k < nums .length; k ++) {
//         if (nums[i] + nums[k] === target) {
//           indexes.push(i, k);
//         }
//       }
//     }

//     return indexes;
// };

var twoSum = function(nums, target) {
  const map = new Map();

  for (let i = 0; i < nums.length; i++) {
    const addend = target - nums[i];

    if (map.has(addend)) {
      return [map.get(addend), i];
    }

    map.set(nums[i], i);
  }
}

console.log(twoSum([3,2,4], 6));
console.log(twoSum([2,7,11,15], 9));
