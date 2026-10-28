class Solution:
    def maxArea(self, h: List[int]) -> int:
        l = 0
        r = len(h) - 1
        area = 0
        while l <= r:
            _area = min(h[l], h[r]) * ( r - l )
            area = max(area, _area)
            if h[l] < h[r]:
                l = l + 1
            else:
                r = r - 1
        return area