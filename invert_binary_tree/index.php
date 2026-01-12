<?php

class TreeNode {
    public $val = null;
    public $left = null;
    public $right = null;
    function __construct($val = 0, $left = null, $right = null) {
        $this->val = $val;
        $this->left = $left;
        $this->right = $right;
    }
}

  // make a queue to store nodes and put root in it
  // while the queue is not empty:
    // get the first (shift, not pop) node from queue
    // swap left/right
    // if there is currNode.left or currNode.right
      // add those to queue
  // return root

function invertTree($root){
  if(is_null($root)){
    return null;
  }

  $queue = [$root];
  
  while(!empty($queue)){
    $node = array_shift($queue);

    $temp = $node->left;
    $node->left = $node->right;
    $node->right = $temp;

    if(!is_null($node->right)){
      $queue[] = $node->right;
    }
    if(!is_null($node->left)){
      $queue[] = $node->left;
    }
  }

  return $root;
}

$node9 = new TreeNode(9);
$node6 = new TreeNode(6);
$node3 = new TreeNode(3);
$node1 = new TreeNode(1);
$node7 = new TreeNode(7, $node6, $node9);
$node2 = new TreeNode(2, $node1, $node3);
$node4 = new TreeNode(4, $node2, $node7);

var_dump($node4);
var_dump('inverted: ',invertTree($node4));
