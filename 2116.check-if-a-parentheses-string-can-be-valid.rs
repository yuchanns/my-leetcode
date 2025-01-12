// @leet start
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let s = s.as_bytes();
        let locked = locked.as_bytes();

        let mut min_count = 0;
        let mut max_count = 0;

        for i in 0..s.len() {
            if locked[i] == b'1' {
                if s[i] == b'(' {
                    min_count += 1;
                    max_count += 1;
                } else {
                    min_count -= 1;
                    max_count -= 1;
                }
            } else {
                min_count -= 1;
                max_count += 1;
            }

            if max_count < 0 {
                return false;
            }
            min_count = min_count.max(0);
        }

        min_count == 0
    }
}
// @leet end

