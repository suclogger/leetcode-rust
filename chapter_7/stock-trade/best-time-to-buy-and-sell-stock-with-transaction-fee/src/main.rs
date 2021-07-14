fn main() {
    assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
}

/**
老套路了，还是用两个状态方程表示

hold
sold

hold[i] = max(
    hold[i-1],
    sold[i - 1] - price[i]
)

sold[i] = max(
    sold[i-1],
    hold[i - 1] + price[i] - fee
)

**/

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut hold = vec![0; prices.len() + 1];
    let mut sold = vec![0; prices.len() + 1];
    hold[0] = i32::min_value() + fee;
    sold[0] = 0;
    sold[1] = i32::min_value();
    for i in 1..=prices.len() {
        hold[i] = std::cmp::max(
            hold[i-1],
            sold[i - 1] - prices[i - 1]
        );
        sold[i] = std::cmp::max(
            sold[i-1],
            hold[i - 1] + prices[i - 1] - fee
        );
    }
    sold[prices.len()]
}