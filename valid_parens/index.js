/**
 * @param {string} s
 * @return {boolean}
 */
 function isValid(s) {
   const map = {
     '(': ')',
     '[': ']',
     '{': '}'
   }
   const stack = [];

   for (const char of s) {
     if (map[char]) {
       stack.push(map[char]);
     } else {
       if (stack.pop() !== char) {
         return false;
       }
     }
   }

   return stack.length === 0;
 };

console.log('should be true: ' , isValid('{}'));
console.log('should be true: ' , isValid('[]'));
console.log('should be true: ' , isValid('()'));
console.log('should be true: ' , isValid('[()]'));
console.log('should be true: ' , isValid('()[]{}'));
console.log('should be false: ' , isValid('[]()]'));
console.log('should be false: ' , isValid('[(])'));
