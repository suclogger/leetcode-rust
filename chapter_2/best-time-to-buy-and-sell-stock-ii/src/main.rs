fn main() {
    let v = vec![7,1,5,3,6,4];
    assert_eq!(max_profit(v), 7);
    let v = vec![2,1,2,0,1];
    assert_eq!(max_profit(v), 2);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 1 {
        return 0;
    }
    let mut pro = 0;
    let mut hold_price = -1;
    for i in 0..prices.len()-1 {
        if prices[i] < prices[i+1] && hold_price < 0{
            hold_price = prices[i];
        }
        if prices[i] > prices[i+1] && hold_price >= 0{
            pro += prices[i] - hold_price;
            hold_price =-1;
        }
    }
    if hold_price >= 0 {
        pro += prices[prices.len()-1] - hold_price;
    }
    pro
}