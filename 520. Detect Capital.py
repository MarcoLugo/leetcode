# Given a word, you need to judge whether the usage of capitals in it is right or not.
#
# We define the usage of capitals in a word to be right when one of the following cases holds:
#
#    All letters in this word are capitals, like "USA".
#    All letters in this word are not capitals, like "leetcode".
#    Only the first letter in this word is capital, like "Google".
#
# Otherwise, we define that this word doesn't use capitals in a right way. 

class Solution:
    def detectCapitalUse(self, word: str) -> bool:
        result = False
        count = 0
        
        for c in word:
            if self.is_capital_letter(c):
                count += 1
        
        if count == 0:
            result = True
        elif count == len(word):
            result = True
        elif count == 1 and self.is_capital_letter(word[0]):
            result = True
        
        return result
                
    @staticmethod
    def is_capital_letter(c):
        return 65 <= ord(c) <= 90
