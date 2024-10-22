var mergeTwoLists = function(list1, list2) {
    return merge_rec(list1, list2);
};

function merge_rec(list1, list2) {
    if (list1 === null) {
        return list2;
    }
    if (list2 === null) {
        return list1;
    }

    if (list1.val < list2.val) {
        list1.next = merge_rec(list1.next, list2);
        return list1;
    } else {
        list2.next = merge_rec(list1, list2.next);
        return list2;
    }
}

