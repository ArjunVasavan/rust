// @leet start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {

        let mut x = 0;

        for &num in &nums {
            x ^= num;
        }
        x 
    }
}
// @leet end

