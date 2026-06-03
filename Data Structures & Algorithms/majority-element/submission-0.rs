impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut _num = nums;
        _num.sort();
        return _num[_num.len() / 2];
    }
}
