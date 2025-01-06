// @leet start
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes = boxes.as_bytes();
        let mut res = vec![0; n];

        let mut ltotal = 0;
        let mut lsteps = 0;

        let mut rtotal = 0;
        let mut rsteps = 0;

        for i in 0..n {
            if boxes[i] == b'0' {
                continue;
            }
            rtotal += 1;
            rsteps += i as i32;
        }

        for i in 0..n {
            res[i] = lsteps + rsteps;

            if boxes[i] == b'1' {
                rtotal -= 1;
                ltotal += 1;
            }

            lsteps += ltotal;
            rsteps -= rtotal;
        }

        res
    }
}
// @leet end
