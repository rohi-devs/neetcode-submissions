class Solution:
    def maxProfit(self, p: List[int]) -> int:
        minBuy = p[0]
        maxP   = 0

        for sell in p:
            maxP   = max(maxP, sell - minBuy)
            minBuy = min(sell, minBuy)
        
        return maxP