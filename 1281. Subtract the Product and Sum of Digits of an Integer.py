# Given an integer number n, return the difference between the product of its digits and the sum of its digits.

class Solution:
    def subtractProductAndSum(self, n: int) -> int:    
        num = str(n)
        product = 1
        difference = 0
        
        for d in num:
            d = int(d)
            product *= d
            difference += d
            
        return product - difference
