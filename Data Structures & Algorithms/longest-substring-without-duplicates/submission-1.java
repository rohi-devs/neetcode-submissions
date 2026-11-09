
class Solution {
    public int lengthOfLongestSubstring(String s) {
        int result = 0;
        int n = s.length();
        for (int i = 0; i < n; i++) {
            HashSet<Character> hs = new HashSet<>();
            for (int j = i; j < n; j++) {
                if (hs.contains(s.charAt(j))) {
                    break;
                }
                hs.add(s.charAt(j));
            }
            result = Math.max(result, hs.size());
        }
        return result;
    }
}
