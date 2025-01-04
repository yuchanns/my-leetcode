// @leet start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if nums.len() < 3 {
            return result;
        }

        let mut nums = nums;
        nums.sort();
        if nums[0] > 0 || nums[nums.len() - 1] < 0 {
            return result;
        }

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let rfirst = -nums[i];
            let mut second = i + 1;
            let mut third = nums.len() - 1;

            while second < third {
                let sum = nums[second] + nums[third];

                if sum == rfirst {
                    result.push(vec![nums[i], nums[second], nums[third]]);

                    while second < third && nums[second] == nums[second + 1] {
                        second += 1;
                    }
                    while second < third && nums[third] == nums[third - 1] {
                        third -= 1;
                    }

                    second += 1;
                    third -= 1;
                } else if sum < rfirst {
                    second += 1;
                } else {
                    third -= 1;
                }
            }
        }

        result
    }
}
// @leet end

