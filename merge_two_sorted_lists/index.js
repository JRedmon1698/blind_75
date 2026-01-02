/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }

 Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]
 */
/**
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */

 function ListNode(val, next) {
  this.val = val === undefined ? 0 : val;
  this.next = next === undefined ? null : next;
}

function mergeTwoLists(list1, list2) {
  if (!list1 && !list2) return null;
  if (!list1) return list2;
  if (!list2) return list1;

  let dummy = new ListNode(0);
  let currentNode = dummy;

  let list1Node = list1;
  let list2Node = list2;

  while (list1Node && list2Node) {
    if (list1Node.val <= list2Node.val) {
      currentNode.next = list1Node;
      list1Node = list1Node.next;
    } else {
      currentNode.next = list2Node;
      list2Node = list2Node.next;
    }

    currentNode = currentNode.next;
  }

  currentNode.next = list1 !== null ? list1 : list2;

  return dummy.next;
}

const list1Node3 = new ListNode(4, null);
const list1Node2 = new ListNode(2, list1Node3);
const list1Node1 = new ListNode(1, list1Node2);
const list2Node3 = new ListNode(4, null);
const list2Node2 = new ListNode(3, list2Node3);
const list2Node1 = new ListNode(1, list2Node2);

console.log(mergeTwoLists(list1Node1, list2Node1));

