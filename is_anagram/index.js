/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function(s, t) {
  if (s.length !== t.length) return false;

  const sMap = {};
  const tMap = {};

  for (let i = 0; i < s.length; i++) {
    if (!sMap[s[i]]) {
      sMap[s[i]] = 1;
    } else {
      sMap[s[i]] += 1;
    }

    if (!tMap[t[i]]) {
      tMap[t[i]] = 1;
    } else {
      tMap[t[i]] += 1;
    }
  }

  const tKeys = Object.keys(tMap);
  const sKeys = Object.keys(sMap);

  if (tKeys.length !== sKeys.length) return false;

  for (const key of tKeys) {
    if (!sMap.hasOwnProperty(key)) return false;
    if (tMap[key] !== sMap[key]) return false;
  }

  return true;
};

let s = 'nagaram';
let t = 'anagram';
console.log(isAnagram(s, t));

s = 'cat';
t = 'car';
console.log(isAnagram(s, t));

