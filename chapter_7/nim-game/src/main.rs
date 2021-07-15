fn main() {
    assert_eq!(can_win_nim(4), false);
}
/**

定义dp数组，dp[i]代表给定i石头下能否胜出

dp[i] = true 仅在3种情况下成立
1: i是手上第1个球: dp[i-1] == false
2: i是手上第2个球: dp[i-1] == true && dp[i-2] == false
3: i是手上第3个球: dp[i-2] == true && dp[i-3] == false

**/

pub fn can_win_nim(n: i32) -> bool {
    if n < 4 {
        return true;
    }
    let mut pre_1 = true;
    let mut pre_2 = true;
    let mut pre_3 = true;
    let mut  cur = true;
    for _ in 4..=n as usize {
        cur = !pre_1 || (pre_1 && !pre_2) || (pre_2 && !pre_3);
        pre_3 = pre_2;
        pre_2 = pre_1;
        pre_1 = cur;
    }
    cur
}