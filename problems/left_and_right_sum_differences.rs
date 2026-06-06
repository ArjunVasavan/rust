// @leet start
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {

        let n = nums.len();

        let mut result = vec![0;n];

        let mut sum = 0;
        for i in 0..n {
            result[i] = sum;
            sum+=nums[i];
        }

        sum = 0;

        for i in (0..n).rev() {
            result[i] = (result[i] - sum).abs();
            sum += nums[i];
        }
        result
    }
}
// @leet end

