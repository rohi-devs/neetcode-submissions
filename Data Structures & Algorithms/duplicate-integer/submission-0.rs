use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut res : HashSet<i32> = HashSet::new();
        for num in nums {
            if !res.insert(num) {
                return true;
            }
        }
        return false;
    }
}
