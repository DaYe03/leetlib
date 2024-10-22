var maxProfit = function(prices) {
    let max = 0;
    for ( let i = 0; i < prices.length; i++){
        for (let j = i + 1; j < prices.length; j++){
            if (prices[i] < prices[j] && max < (prices[j] - prices[i]) ) {
                max = prices[j] - prices[i];
            }
        }
    }
    return max;
};

