<?php

class ListNode {
    public $val = 0;
    public $next = null;
    function __construct($val = 0, $next = null) {
        $this->val = $val;
        $this->next = $next;
    }
}

/**
 * @param ListNode $list1
 * @param ListNode $list2
 * @return ListNode
 */
function mergeTwoLists($list1, $list2) {
    if(is_null($list1) && is_null($list2)){
        return null;
    }
    if(is_null($list1)){
        return $list2;
    }
    if(is_null($list2)){
        return $list1;
    }

    $dummy = new ListNode(0);
    $current = $dummy;

    $list1Node = $list1;
    $list2Node = $list2;

    while(!is_null($list1Node) && !is_null($list2Node)){
        if($list1Node->val <= $list2Node->val){
            $current->next = $list1Node;
            $list1Node = $list1Node->next;
        }
        else {
            $current->next = $list2Node;
            $list2Node = $list2Node->next;
        }

        $current = $current->next;
    }

    $current->next = !is_null($list1Node) ? $list1Node : $list2Node;

    return $dummy->next;
}

$list1Node3 = new ListNode(4, null);
$list1Node2 = new ListNode(2, $list1Node3);
$list1Node1 = new ListNode(1, $list1Node2);

$list2Node3 = new ListNode(4, null);
$list2Node2 = new ListNode(3, $list2Node3);
$list2Node1 = new ListNode(1, $list2Node2);

var_dump(mergeTwoLists($list1Node1, $list2Node1));
