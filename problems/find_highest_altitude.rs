// @leet start
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut output = 0;
        let mut accumulate = 0;

        for num in gain {

            accumulate += num;
            output = output.max(accumulate);
        }
       output 
    }
}
// @leet end

