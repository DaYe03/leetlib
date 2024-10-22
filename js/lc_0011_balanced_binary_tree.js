/**
 * Definition for a binary tree node.
 */
function TreeNode(val, left, right) {
    this.val = (val === undefined ? 0 : val)
    this.left = (left === undefined ? null : left)
    this.right = (right === undefined ? null : right)
}
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isBalanced = function (root) {
    if (root == null){
        return true;
    }
    let leftH = deepHeight(root.left, 1);
    let rightH = deepHeight(root.right, 1);
    return Math.abs(leftH - rightH) <= 1 && isBalanced(root.left) && isBalanced(root.right);
};

function deepHeight (node, h){
    if (node === null) {
        return h;
    }    
    let leftH = deepHeight(node.left, h+1);
    let rightH = deepHeight(node.right, h+1);
    return leftH > rightH ? leftH : rightH;
}