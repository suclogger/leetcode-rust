fn main() {
    assert_eq!(number_of_weeks(vec![1,2,3]), 6);
    assert_eq!(number_of_weeks(vec![5,2,1]), 7);
}
/**
不同于别的分配任务类的题目，这题要求的返回是总数即可，按照正常的思路用priority_queue实际提交的时候会TLE
其实就考虑所有任务中耗时最多的那个，有两种情况，一种是没超过半数，一种是超过了半数
如果没超过半数，说明可以全部岔开，任务都可以完成
如果超过了半数，(n/2 - (x - n/2)) * 2 = (n - x) * 2
OOOOOOOOO OOO|OOO
XXXXYYYZZ    |

**/

pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut max: i64 = 0;
    let mut sum: i64 = 0;
    for ms in milestones {
        max = std::cmp::max(max, ms as i64);
        sum = sum + (ms as i64);
    }
    return if max <= sum / 2 {
        sum as i64
    } else {
        (sum - max ) * 2 + 1
    }
}