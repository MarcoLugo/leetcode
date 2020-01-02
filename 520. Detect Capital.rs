// Given a word, you need to judge whether the usage of capitals in it is right or not.
//
// We define the usage of capitals in a word to be right when one of the following cases holds:
//
//    All letters in this word are capitals, like "USA".
//    All letters in this word are not capitals, like "leetcode".
//    Only the first letter in this word is capital, like "Google".
//
// Otherwise, we define that this word doesn't use capitals in a right way. 


impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut count = 0;
        let mut result = false;
        
        for c in word.as_bytes().into_iter() {
            if Solution::is_capital_letter(*c) {
                count += 1;
            }
        }
        
        if (count == 0) || (count == word.len()) {
            result = true;
        } else if (count == 1) && (Solution::is_capital_letter(word.as_bytes()[0])) {
            result = true;
        }
        
        result
    }
    
    fn is_capital_letter(c: u8) -> bool {
        (c >= 65) && (c <= 90)
    }
}