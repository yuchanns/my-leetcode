// @leet start
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut first = [u32::MAX; 26];
        let mut last = [0; 26];

        for (i, &c) in s.iter().enumerate() {
            let i = i as u32;
            let idx = (c - b'a') as usize;
            first[idx] = first[idx].min(i);
            last[idx] = last[idx].max(i);
        }

        let mut result = 0;
        for i in 0..26 {
            if first[i] >= last[i] {
                continue;
            }
            let mut seen = 0u32;
            for j in first[i] + 1..last[i] {
                seen |= 1 << (s[j as usize] - b'a');
            }
            let mut count = 0;
            while seen != 0 {
                seen &= seen - 1;
                count += 1;
            }
            result += count;
        }

        result
    }
}
// @leet end
