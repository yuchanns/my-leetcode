// @leet start
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut result = vec![0; n];
        let mut count = vec![0; n + 1];
        let mut common = 0;

        for i in 0..n {
            count[a[i] as usize] += 1;
            if count[a[i] as usize] == 2 {
                common += 1;
            }

            count[b[i] as usize] += 1;
            if count[b[i] as usize] == 2 {
                common += 1;
            }

            result[i] = common;
        }

        result
    }
}
// @leet end

