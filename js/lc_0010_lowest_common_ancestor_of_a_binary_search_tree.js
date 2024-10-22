/**
 * Definition for a binary tree node.
 */
function TreeNode(val) {
    this.val = val;
    this.left = this.right = null;
}

/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
    let pAncestors = ancestors(root, p);
    let qAncestors = ancestors(root, q);
    let lowestAncestor = -1;
    for (let i = 0; i< qAncestors.length; i++){
        if (qAncestors[i] !== pAncestors[i]){
            break;
        }
        lowestAncestor = qAncestors[i];
    }
    return lowestAncestor;
};

function ancestors(node, p){
    if (node !== null){
        let ancestor = [node];
        if (node === p) {
            return ancestor;
        }
        let left = ancestors(node.left, p);
        let right = ancestors(node.right, p);
        if (left.length > 0){
            return ancestor.concat(left);
        }else if (right.length > 0){
            return ancestor.concat(right);
        }
    }
    return [];
}