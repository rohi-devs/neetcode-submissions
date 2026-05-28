impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..2 {
            for &val in &nums {
                result.push(val);
            }
        }
        result
    }
}
