// @leet start
impl Solution {
    pub fn is_prefix_and_suffix(w1: &str, w2: &str) -> bool {
        let len1 = w1.len();
        let len2 = w2.len();

        if len1 > len2 {
            return false;
        }

        w1 == &w2[..len1] && w1 == &w2[len2 - len1..]
    }

    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        let n = words.len();

        for i in 0..n {
            for j in i + 1..n {
                if Self::is_prefix_and_suffix(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }
        count
    }
}
// @leet end

