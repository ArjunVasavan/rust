// @leet start
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {

        let mn = *nums.iter().min().unwrap();
        let mx = *nums.iter().max().unwrap();
        (mx - mn) as i64 * k as i64
    }
}
// @leet end

