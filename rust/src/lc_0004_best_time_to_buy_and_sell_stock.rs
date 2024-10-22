pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cheaper: i32 = 10001;
    let mut max_profit = 0;
    for price in prices {
        if cheaper > price {
            cheaper = price;
        } else if max_profit < (price - cheaper) {
            max_profit = price - cheaper;
        }
    }
    return max_profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example 1: prices = [7,1,5,3,6,4]
    #[test]
    fn test_max_profit_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = max_profit(prices);
        assert_eq!(result, 5); // Max profit is 5
    }

    // Example 2: prices = [7,6,4,3,1]
    #[test]
    fn test_max_profit_case_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = max_profit(prices);
        assert_eq!(result, 0); // No transactions possible, max profit is 0
    }
}
