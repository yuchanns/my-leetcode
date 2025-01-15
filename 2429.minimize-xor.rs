// @leet start
impl Solution {
    pub fn count_ones(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        count
    }
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let count = Solution::count_ones(num2);
        let mut result = 0;

        if count == Solution::count_ones(num1) {
            return num1;
        }

        let mut num1_bits = vec![0; 32];
        for i in 0..32 {
            num1_bits[i] = (num1 >> i) & 1;
        }

        let mut remaining = count;

        for i in (0..32).rev() {
            if remaining > 0 && num1_bits[i] == 1 {
                result |= 1 << i;
                remaining -= 1;
            }
        }

        if remaining > 0 {
            for i in 0..32 {
                if remaining > 0 && num1_bits[i] == 0 {
                    result |= 1 << i;
                    remaining -= 1;
                }
            }
        }

        result
    }
}
// @leet end
