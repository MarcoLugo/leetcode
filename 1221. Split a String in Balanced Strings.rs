// Balanced strings are those who have equal quantity of 'L' and 'R' characters.
// Given a balanced string s split it in the maximum amount of balanced strings.
// Return the maximum amount of splitted balanced strings.

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut buffer_count = 0;
        
        for c in s.chars() {
            buffer_count = match c {
                'L' => buffer_count + 1,
                'R' => buffer_count - 1,
                _ => buffer_count
            };
            
            if buffer_count == 0 {
                count += 1;
            }
        }
        
        count
    }
}