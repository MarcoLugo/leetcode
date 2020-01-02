# Implement function ToLowerCase() that has a string parameter str, and returns the same string in lowercase.

class Solution:
    def toLowerCase(self, str: str) -> str:
        s = [ord(c) for c in str]
        
        for i, c in enumerate(s):
            print(c)
            if 65 <= c <= 90:  # if ASCII uppercase
                s[i] = chr(s[i] + 32)
            else:
                s[i] = chr(c)
                
        return ''.join(s)
