<?php

/**
 * @param Integer[] $prices
 * @return Integer
 */
function maxProfit($prices) {
    $buy = $prices[0];
    $profit = 0;

    foreach($prices as $price){
      if($price < $buy){
        $buy = $price;
      }
      else if($price - $buy > $profit){
        $profit = $price - $buy;
      }
    }

    return $profit;
}

var_dump(maxProfit([7,1,5,3,6,4]));
var_dump(maxProfit([7,6,4,3,1]));
