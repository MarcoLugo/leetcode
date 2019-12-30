# Balanced strings are those who have equal quantity of 'L' and 'R' characters.
# Given a balanced string s split it in the maximum amount of balanced strings.
# Return the maximum amount of splitted balanced strings.

class Solution:
    def balancedStringSplit(self, s: str) -> int:
        buffer_count = 0
        count = 0
        
        for c in s:
            if c == 'L':
                buffer_count += 1
            elif c == 'R':
                buffer_count -= 1
            
            if buffer_count == 0:
                count += 1
        
        return count
