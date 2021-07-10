use std::cmp::max;

fn main() {
    assert_eq!(max_profit(vec![1,2,3,0,2]), 3);
}

/**
典型的使用DP解决股票交易题型的题型，不同之处是相比之前的持有，售出，多了一个冷却的状态

hold[i] = max(
    hold[i-1],
    cooled[i-1] - prices[i]
)

sold[i] =  hold[i-1] + prices[i]

cooled[i] = max(
    cooled[i-1],
    sold[i-1]
)

再考虑边界
hold[0] = NA
sold[0] = NA
cooled[0] = 0

最后考虑题解
最优解只可能在sold和cooled中存在，遍历比较得出答案

**/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let max_price = prices.iter().max().unwrap();
    let mut hold = vec![i32::min_value() + max_price; prices.len() + 1];
    let mut sold = vec![i32::min_value() + max_price; prices.len() + 1];
    let mut cooled = vec![0; prices.len() + 1];
    for i in 1..=prices.len() {
        hold[i] = max(
            hold[i-1],
            cooled[i-1] - prices[i-1]
        );

        sold[i] =  hold[i-1] + prices[i-1];

        cooled[i] = max(
            cooled[i-1],
            sold[i-1]
        );
    }
    let mut ans = 0;
    for i in 1..=prices.len() {
        ans = max(max(ans, cooled[i]), sold[i]);
    }
    ans
}