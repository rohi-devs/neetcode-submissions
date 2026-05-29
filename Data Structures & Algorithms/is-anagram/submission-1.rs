use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut _a = HashMap::new();
        for ch in s.chars() {
            *_a.entry(ch).or_insert(0) += 1;
        }
        let mut _b = HashMap::new();
        for ch in t.chars() {
            *_b.entry(ch).or_insert(0) += 1;
        }
        return _a == _b;
    }
}
