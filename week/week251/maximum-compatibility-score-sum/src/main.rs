fn main() {
    println!("Hello, world!");
}

/**
暴力求解

**/

pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {

    fn backtrack(m_selected: &mut Vec<bool>, s_idx: usize,
                 students: &Vec<Vec<i32>>, mentors: &Vec<Vec<i32>>,
                 cur_score:i32, score: &mut Vec<i32>, m: usize, n: usize) {
        if s_idx >= m {
            score.push(cur_score);
            return;
        }
        for j in 0..mentors.len() {
            if !m_selected[j] {
                m_selected[j] = true;
                let mut cur_s = 0;
                for k in 0..n {
                    if students[s_idx][k] == mentors[j][k] {
                        cur_s = cur_s + 1;
                    }
                }
                backtrack(m_selected, s_idx + 1, students, mentors, cur_score + cur_s, score, m, n);
                m_selected[j] = false;
            }
        }
    }

    let n = students[0].len();
    let mut scores: Vec<i32> = Vec::new();
    let mut m_s = vec![false; mentors.len()];
    backtrack(&mut m_s, 0, &students, &mentors, 0, &mut scores, students.len(), n);
    *scores.iter().max().unwrap()
}

