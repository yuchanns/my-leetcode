// @leet start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let mut last_seen = [-1; 128];

        for (end, ch) in s.chars().enumerate() {
            let idx = ch as i32;
            start = start.max(last_seen[idx as usize] + 1) as i32;
            last_seen[idx as usize] = end as i32;
            max_len = max_len.max(end as i32 - start + 1);
        }

        max_len
    }
}
// @leet end

