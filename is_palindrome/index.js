/**
 * @param {string} s
 * @return {boolean}
 */

var isPalindrome = function(s) {
  let rev = '';

  for (let i = s.length - 1; i >=0; i--) {
    rev += s[i];
  }

  const trimmedS = s.toLowerCase().replace(/[^a-z0-9]/g, '');
  const trimmedRev = rev.toLowerCase().replace(/[^a-z0-9]/g, '');

  return trimmedS === trimmedRev;
};


var isPalindrome = function(s) {
  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    // ignore anything that's not a number or letter
    if (!/[a-z0-9]/i.test(s[left])) {
      left++;
    } else if (!/[a-z0-9]/i.test(s[right])) {
      right--;
    } else {
      if (s[left].toLowerCase() !== s[right].toLowerCase()) {
        return false;
      }

      left++;
      right--;
    }
  }

  return true;
};

console.log(isPalindrome('asddsa'));
console.log(isPalindrome("Was it a rat I saw"));
console.log(isPalindrome('asdsa'));
console.log(isPalindrome('asdsaa'));
console.log(isPalindrome('A man, a plan, a canal: Panama'));
