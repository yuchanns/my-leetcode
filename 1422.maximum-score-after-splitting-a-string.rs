// @leet start
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max_score = 0;
        let mut left_zeros = 0;
        let mut right_ones = s.chars().filter(|&c| c == '1').count() as i32;

        for (i, c) in s.chars().enumerate() {
            if i == s.len() - 1 {
                break;
            }
            if c == '0' {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
            max_score = max_score.max(left_zeros + right_ones);
        }

        max_score
    }
}
// @leet end

