impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 || k == 0 {
            return 0;
        }

        let k = k as usize;
        if k >= n / 2 {
            return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
        }

        let mut buy: Box<[i32]> = (0..k).map(|_| i32::MIN / 2).collect();
        let mut sell: Box<[i32]> = (0..k).map(|_| 0).collect();

        for &price in &prices {
            buy[0] = buy[0].max(-price);
            sell[0] = sell[0].max(buy[0] + price);

            for j in 1..k {
                buy[j] = buy[j].max(sell[j - 1] - price);
                sell[j] = sell[j].max(buy[j] + price);
            }
        }

        sell[k - 1]
    }
}