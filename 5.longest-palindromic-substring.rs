// @leet start
impl Solution {
    pub fn expand(chars: &[char], left: isize, right: isize) -> (isize, isize) {
        let n = chars.len() as isize;
        let (mut l, mut r) = (left, right);
        while l >= 0 && r < n && chars[l as usize] == chars[r as usize] {
            l -= 1;
            r += 1;
        }
        (l + 1, r - l - 1)
    }

    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (mut start, mut len) = (0, 0);
        let n = chars.len();
        for i in 0..n {
            let (s, l) = Solution::expand(&chars, i as isize, i as isize);
            if l > len {
                start = s;
                len = l;
            }
            if i < n - 1 && chars[i] == chars[i + 1] {
                let (s, l) = Solution::expand(&chars, i as isize, (i + 1) as isize);
                if l > len {
                    start = s;
                    len = l;
                }
            }
        }

        chars[start as usize..(start + len) as usize]
            .iter()
            .collect()
    }
}
// @leet end

