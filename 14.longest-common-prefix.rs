// @leet start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first = strs[0].as_bytes();
        let mut len = first.len();

        for s in &strs[1..] {
            let bytes = s.as_bytes();
            len = len.min(bytes.len());

            for (i, &b) in first[..len].iter().enumerate() {
                if b != bytes[i] {
                    len = i;
                    break;
                }
            }

            if len == 0 {
                return String::new();
            }
        }
        String::from_utf8_lossy(&first[..len]).into_owned()
    }
}
// @leet end

