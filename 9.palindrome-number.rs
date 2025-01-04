// @leet start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut x = x as u32;
        let mut reserved = 0u32;

        while x > reserved {
            reserved = reserved * 10 + x % 10;
            x /= 10;
        }
        x == reserved || x == reserved / 10
    }
}
// @leet end

