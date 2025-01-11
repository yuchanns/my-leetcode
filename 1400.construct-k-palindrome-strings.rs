// @leet start
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if k > s.len() as i32 {
            return false;
        }

        let mut freq = [0; 26];
        for &ch in s.as_bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        let odd_count = freq.iter().filter(|&&count| count % 2 == 1).count() as i32;

        if k < odd_count {
            return false;
        }

        true
    }
}
// @leet end

