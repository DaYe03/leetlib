pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut r = Vec::new();
    let mut found = false;
    for (index, num) in nums.iter().enumerate() {
        let compliment = target - *num;
        for (index_2, num) in &mut nums[index + 1..].iter().enumerate() {
            if *num == compliment {
                r.push(index as i32);
                r.push((index_2 + index + 1) as i32);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let input = [3, 2, 4];
        let target = 6;
        let actual = two_sum(input.to_vec(), target);
        let expected = [1, 2].to_vec();
        assert_eq!(expected, actual);
    }
}
