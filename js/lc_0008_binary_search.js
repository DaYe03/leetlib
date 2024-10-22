var search = function(nums, target) {
    return helper(nums, target, 0)
};

function helper (nums, target, initIndex) {
    let half = Math.floor(nums.length /2);
    if (half == 0 ){
        if (nums[half] == target) {
            return initIndex;
        }else{
            return -1;
        } 
    }else if (nums[half] > target){
        return helper(nums.slice(0,half), target, initIndex);
    } else {
        return helper(nums.slice(half), target, initIndex + half);
    }
}