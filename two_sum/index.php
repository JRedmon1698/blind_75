<?php

// use Ds\Map;

// function twoSum($aNums, $iTarget){
//   $mMap = new Map();

//   foreach($aNums as $iIdx => $iNum){
//     $iAddend = $iTarget - $iNum;

//     if($mMap->hasKey($iAddend)){
//       return [$mMap->get($iAddend), $iIdx];
//     }
//     else {
//       $mMap->put($iNum, $iIdx);
//     }
//   }
// }

function twoSum($aNums, $iTarget){
  $oMap = [];

  foreach($aNums as $iIdx => $iNum){
    $iAddend = $iTarget - $iNum;
    if(isset($oMap[$iAddend])){
      return [$oMap[$iAddend], $iIdx];
    }
    else {
      $oMap[$iNum] = $iIdx;
    }
  }
}

var_dump(twoSum([3,2,4], 6));
var_dump(twoSum([2,7,11,15], 9));
