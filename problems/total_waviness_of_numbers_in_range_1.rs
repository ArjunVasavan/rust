// @leet start
impl Solution {

    fn foo(num: i32) -> i32 {
        let mut arr = Vec::new();
        let mut temp = num;

        while temp > 0 {
            arr.push(temp%10);
            temp /= 10;
        }

        let size = arr.len();
        let mut count = 0;

        for i in 1..size-1 { /* skips first and last */
            if arr[i] > arr[i-1] && arr[i] > arr[i+1] {
                count+=1;
            } else if arr[i] < arr[i-1] && arr[i] < arr[i+1] {
                count+=1;
            }
        }
        count
    }

    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
     
        (num1..=num2)
            .filter(|&i| i >= 100 )
            .map(|i| Solution::foo(i))
            .sum()

    }
}
// @leet end


fn main() {
    
}
