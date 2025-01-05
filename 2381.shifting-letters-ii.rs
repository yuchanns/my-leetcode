// @leet start
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut diff = vec![0i32; n + 1];

        for shift in shifts.iter() {
            let start = shift[0] as usize;
            let end = shift[1] as usize;
            let delta = if shift[2] == 1 { 1 } else { -1 };
            diff[start] += delta;
            diff[end + 1] -= delta;
        }

        let mut result = s.into_bytes();
        let mut final_shift = 0;

        for i in 0..n {
            final_shift += diff[i];
            let shift = ((final_shift % 26 + 26) % 26) as i32;
            let final_pos = ((result[i] - b'a') as i32 + shift) % 26;
            result[i] = (final_pos as u8 + b'a');
        }

        String::from_utf8(result).unwrap()
    }
}
// @leet end

