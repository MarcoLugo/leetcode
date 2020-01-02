# Given a string s consists of upper/lower-case alphabets and empty space
# characters ' ', return the length of last word (last word means the last
# appearing word if we loop from left to right) in the string.
#
# If the last word does not exist, return 0.
#
# Note: A word is defined as a maximal substring consisting of non-space characters only.

class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        words = s.split(' ')
        words = filter(lambda s: len(s) > 0, words)
        words = list(words)
        
        if len(words) == 0:
            return 0
        else:
            return len(words[-1])
