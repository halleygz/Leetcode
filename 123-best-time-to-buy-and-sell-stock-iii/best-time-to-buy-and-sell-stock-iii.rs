impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut first = vec![0; n];
        let mut second = vec![0; n+1];

        let mut first_price = i32::MAX;
        let mut first_profit = i32::MIN;

        let mut second_price = i32::MIN;
        let mut second_profit = i32::MIN;

        for i in 0..n {
            first_price = first_price.min(prices[i]);
            first_profit = first_profit.max(prices[i]-first_price);
            first[i] = first_profit;

            second_price = second_price.max(prices[n-i-1]);
            second_profit = second_profit.max(second_price - prices[n-1-i]);
            second[n-1-i] = second_profit;
        }

        let mut max_profit = 0;
        for i in 0..n {
            max_profit = max_profit.max(first[i]+second[i+1])
        }
        max_profit
    }
}