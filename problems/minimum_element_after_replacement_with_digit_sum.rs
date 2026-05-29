// @leet start
impl Solution {
    pub fn sum_of_digits(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result += (n % 10);
            n /=10;
        }
        result /* if we dont put ; it will impicitely return result */
    }
    pub fn min_element(nums: Vec<i32>) -> i32 {

        let mut min = i32::MAX;
        for  n in  &nums  { /* & to borrow not to consume */
            let mut check = Self::sum_of_digits(*n); /* * for dereferencing n */
            if check < min{
                min = check;
            }
        }

        min
    }
}
// @leet end
