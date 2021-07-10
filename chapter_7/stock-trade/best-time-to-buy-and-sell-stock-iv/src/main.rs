use std::cmp::max;

fn main() {
    assert_eq!(max_profit(2, vec![2,4,1]), 2);
    assert_eq!(max_profit(2, vec![3,2,6,5,0,3]), 7);
}

/**
best-time-to-buy-and-sell-stock的进阶版
之前的问题中，hold和sold都是一次性的，如果支持完成k笔交易，需要通过一个向量做扩展

hold[i][k]表示第k次持有在第i天能获取的最大收益
sold[i][k]表示第k次售出在第i天能获取的最大收益

hold[i][k] = max(
    hold[i-1][k],
    sold[i-1][k-1] - price[i]
)

sold[i][k] = max(
    hold[i-1][k] + price[i],
    sold[i-1][k]
)

再考虑下边界
hold[0][0] = 0
hold[0][1..k] = NA
sold[0][0] = 0
sold[0][1..k] = NA

NA的处理方式是通过 i32::min_value() + max_price 确保边界减去price不会溢出

 **/

pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let max_price = prices.iter().max().unwrap();
    let mut hold = vec![vec![i32::min_value() + max_price; k as usize + 1]; prices.len() + 1];
    let mut sold = vec![vec![i32::min_value() + max_price; k as usize + 1]; prices.len() + 1];
    hold[0][0] = 0;
    sold[0][0] = 0;
    for i in 1..=prices.len() {
        for j in 0..=k as usize {
            if j == 0 {
                hold[i][j] = hold[i-1][j];
                sold[i][j] = sold[i-1][j];
            } else {
                hold[i][j] = max(
                    hold[i-1][j],
                    sold[i-1][j-1] - prices[i - 1]
                );
                sold[i][j] = max(
                    hold[i-1][j] + prices[i - 1],
                    sold[i-1][j]
                );
            }
        }
    }

    let mut ans = 0;
    for i in 0..=k as usize {
        ans = max(ans, sold[prices.len()][i]);
    }
    ans
}