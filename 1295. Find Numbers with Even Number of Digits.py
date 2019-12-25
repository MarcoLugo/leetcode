class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        count = 0

        for num in nums:
            inner_count = 0

            while num != 0:
                num = num // 10
                inner_count += 1

            if inner_count % 2 == 0:
                count += 1

        return count
