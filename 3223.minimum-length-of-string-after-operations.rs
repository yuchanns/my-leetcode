// @leet start
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut n = s.len() as i32;
        let mut alpha = vec![0; 26];
        for &c in s.iter() {
            alpha[(c - b'a') as usize] += 1;
        }
        for i in 0..26 {
            while alpha[i] > 2 {
                alpha[i] -= 2;
                n -= 2;
            }
        }
        n
    }
}
// @leet end
