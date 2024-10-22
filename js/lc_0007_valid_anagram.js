/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function(s, t) {
    const map = new Map();
    for(let c of s) {
        if (map.get(c) == undefined){
            map.set(c, 1);
        }else{
            map.set(c, map.get(c)+1);
        }
    }
    for (let c of t){
        if (map.get(c) == undefined){
            return false;
        }else{
            map.set(c, map.get(c)-1);
        }
    }
    for (let i of map.values()){
            if (i !== 0){
                return false;
            }
    }
    return true;
};