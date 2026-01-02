function TreeNode(val, left, right) {
  this.val = (val===undefined ? 0 : val)
  this.left = (left===undefined ? null : left)
  this.right = (right===undefined ? null : right)
}
/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
// var invertTree = function(root) {
//   if (root === null) return null;

//   const queue = [root];
//   let i = 0;

//   while (queue.length) {
//     let node = queue.shift();

//     let temp = node.left;
//     node.left = node.right;
//     node.right = temp;

//     if (node.left) queue.push(node.left);
//     if (node.right) queue.push(node.right);
//   }

//   return root;
// };

var invertTree = function(root) {
  if (root === null) return null;

  const queue = [root];

  for (let i = 0; i < queue.length; i++) {
    let node = queue[i];

    let temp = node.left;
    node.left = node.right;
    node.right = temp;

    if (node.left) queue.push(node.left);
    if (node.right) queue.push(node.right);
  }

  return root;
};

// Input: root = [4,2,7,1,3,6,9]
// Output: [4,7,2,9,6,3,1]
let t1Node9 = new TreeNode(9);
let t1Node6 = new TreeNode(6);
let t1Node3 = new TreeNode(3);
let t1Node1 = new TreeNode(1);
let t1Node7 = new TreeNode (7, t1Node6, t1Node9);
let t1Node2 = new TreeNode(2, t1Node1, t1Node3);
let t1Node4 = new TreeNode(4, t1Node2, t1Node7);

console.log('original tree: ', t1Node4);
console.log(invertTree(t1Node4));

