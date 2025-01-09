// @leet start
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let pref = pref.as_bytes();
        let pref_len = pref.len();
        let mut count = 0;

        for word in words.iter() {
            let word = word.as_bytes();

            if pref_len > word.len() {
                continue;
            }

            let mut is_prefix = true;
            for i in 0..pref_len {
                if pref[i] != word[i] {
                    is_prefix = false;
                    break;
                }
            }

            if is_prefix {
                count += 1;
            }
        }

        count
    }
}
// @leet end

