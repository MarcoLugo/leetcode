// Given a string s consists of upper/lower-case alphabets and empty space
// characters ' ', return the length of last word (last word means the last
// appearing word if we loop from left to right) in the string.
//
// If the last word does not exist, return 0.
//
// Note: A word is defined as a maximal substring consisting of non-space characters only.


impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut result = 0;
        let mut words: Vec<&str> = s.split(' ').filter(|x| x.len() > 0).collect();

        if words.len() > 0 {
            let mut last_word = String::from(words.pop().unwrap());
            result = last_word.len() as i32;
        }
    
    result
    }
}