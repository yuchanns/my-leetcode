// @leet start
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_by(|prev, pos| prev.len().cmp(&pos.len()));
        let mut results = vec![];
        for i in 0..words.len() {
            if results.contains(&words[i]) {
                continue;
            }
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    results.push(words[i].clone());
                    break;
                }
            }
        }
        results
    }
}
// @leet end
