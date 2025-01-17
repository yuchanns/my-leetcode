// @leet start
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut xor_sum = 0;
        let n = derived.len();

        for i in 0..n {
            xor_sum ^= derived[i];
        }

        xor_sum == 0
    }
}
// @leet end

