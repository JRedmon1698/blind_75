<?php

/**
* @param String $s
* @return Boolean
*/
function isPalindromeBasic($s) {
  $strippedOriginal = preg_replace('/[^a-zA-Z0-9]/', '', $s);
  $reversed = '';

  for($i = strlen($s) - 1; $i >= 0; $i--){
    if(ctype_alnum($s[$i])){
      $reversed .= $s[$i];
    }
  }

  return strtolower($strippedOriginal) === strtolower($reversed);
}

function isPalindrome($s){
  $left = 0;
  $right = strlen($s) - 1;

  while($left < $right){
    if(preg_match('/[^a-zA-Z0-9]/', $s[$left])){
      $left++;
    }
    else if(preg_match('/[^a-zA-Z0-9]/', $s[$right])){
      $right--;
    } else {
      if(strtolower($s[$left]) !== strtolower($s[$right])){
        return false;
      }

      $left++;
      $right--;
    }
  }

  return true;
}

var_dump(isPalindrome('asddsa'));
var_dump(isPalindrome('Was it a rat I saw'));
var_dump(isPalindrome('A man, a plan, a canal: Panama'));
var_dump(isPalindrome('asddsaa'));
