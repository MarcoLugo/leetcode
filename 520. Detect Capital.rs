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
        let capital_masks: Vec<_> = word
            .as_bytes()
            .into_iter()
            .map(|&c| Self::is_capital_letter(c))
            .collect();
        let capital_count = capital_masks.iter().filter(|&&x| x).count();

        if capital_count == 0 || capital_count == word.len() {
            true
        } else if capital_count == 1 && capital_masks[0] {
            true
        } else {
            false
        }
    }

    fn is_capital_letter(c: u8) -> bool {
        c >= 65 && c <= 90
    }
}
