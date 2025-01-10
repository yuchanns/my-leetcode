// @leet start
impl Solution {
    fn count_freq(word: &str) -> [i32; 26] {
        let mut freq = [0; 26];
        for &c in word.as_bytes() {
            freq[(c - b'a') as usize] += 1;
        }
        freq
    }

    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut max_freq = [0; 26];
        for word in words2.iter() {
            let curr_freq = Self::count_freq(word);
            for i in 0..26 {
                max_freq[i] = max_freq[i].max(curr_freq[i]);
            }
        }

        words1
            .into_iter()
            .filter(|word| {
                let word_freq = Self::count_freq(word);
                (0..26).all(|i| word_freq[i] >= max_freq[i])
            })
            .collect()
    }
}
// @leet end

