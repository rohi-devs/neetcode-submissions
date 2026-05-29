impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res  : HashMap<String, Vec<String>> = HashMap::new();
        for _str in strs {
            let mut chars : Vec<char> = _str.chars().collect();
            chars.sort();
            let chars : String = chars.into_iter().collect();
            res.entry(chars).or_default().push(_str.clone());
        }
        return res.into_values().collect();
    }
}
