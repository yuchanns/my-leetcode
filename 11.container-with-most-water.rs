// @leet start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0usize;
        let mut right = (height.len() - 1) as usize;

        while left < right {
            let h = height[left].min(height[right]) as usize;
            let area = (right - left) * h;
            max_area = max_area.max(area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area as i32
    }
}
// @leet end

