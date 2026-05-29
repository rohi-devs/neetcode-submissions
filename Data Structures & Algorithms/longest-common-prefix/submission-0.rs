impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let mut prefix = strs[0].clone();
        for _str in strs.iter().skip(1) {
            while !_str.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }
        return prefix;
    }
}
