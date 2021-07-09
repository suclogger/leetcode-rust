use std::cmp::max;

fn main() {
    assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
    assert_eq!(max_profit(vec![7,6,4,3,1]), 0);
}
/**
股票交易类问题通常可以用动态规划来解决。
定义dp数组
hold1[i] = max(
    hold1[i-1] -- 之前已经买入，今天不动
    -prices[i]  -- 之前未买入，今天买入
)
sold1[i] = max(
    hold1[i-1] + prices[i] -- 之前已经买入，今天卖出
    sold1[i-1]   -- 之前已经卖出了
)
再考虑边界条件，如果有余力，做下空间压缩
 **/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut hold = vec![i32::min_value(); prices.len() + 1];
    let mut sold = vec![0; prices.len() + 1];

    for i in 1..=prices.len() {
        hold[i] = max(hold[i - 1], -prices[i - 1]);
        sold[i] = max(hold[i - 1] + prices[i - 1], sold[i-1]);
    }
    sold[prices.len()]
}