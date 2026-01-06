<?php

/**
 * @param String $s
 * @return Boolean
 */
function isValid($sStr){
  $oCharMap = [
    '(' => ')',
    '[' => ']',
    '{' => '}'
  ];

  $aStack = [];

  foreach(str_split($sStr) as $cChar){
    if(isset($oCharMap[$cChar])){
      array_push($aStack, $oCharMap[$cChar]);
    }
    else {
      // unlike js, we need to check if the stack is empty explicitly
      // because array_pop does not "safely" fail
      if(empty($aStack) || array_pop($aStack) !== $cChar){
        return false;
      }
    }
  }

  return empty($aStack);
};

var_dump(isValid("()[]{})"));
var_dump(isValid("()[]{}"));
var_dump(isValid("(]"));
