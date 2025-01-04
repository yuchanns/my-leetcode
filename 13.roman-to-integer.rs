// @leet start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut char_to_int = [0; 26];
        char_to_int[(b'I' - b'A') as usize] = 1;
        char_to_int[(b'V' - b'A') as usize] = 5;
        char_to_int[(b'X' - b'A') as usize] = 10;
        char_to_int[(b'L' - b'A') as usize] = 50;
        char_to_int[(b'C' - b'A') as usize] = 100;
        char_to_int[(b'D' - b'A') as usize] = 500;
        char_to_int[(b'M' - b'A') as usize] = 1000;
        for i in 0..s.len() {
            let next = i + 1;
            let current = char_to_int[(s[i] - b'A') as usize];
            if next < s.len() {
                let next = char_to_int[(s[next] - b'A') as usize];
                if current < next {
                    result -= current;
                } else {
                    result += current;
                }
            } else {
                result += current;
            }
        }
        result
    }
}
// @leet end

