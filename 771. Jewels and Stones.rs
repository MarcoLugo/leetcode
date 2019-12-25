// You're given strings J representing the types of stones that are jewels,
// and S representing the stones you have.  Each character in S is a type
// of stone you have.  You want to know how many of the stones you have
// are also jewels.

// The letters in J are guaranteed distinct, and all characters in J and S
// are letters. Letters are case sensitive, so "a" is considered a different
// type of stone from "A".


use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = HashSet::new();
        let mut count = 0;
        
        // better way of doing this?
        for c in j.chars() {
            jewels.insert(c);
        }
        
        for stone in s.chars() {
            if jewels.contains(&stone) {
                count += 1;
            }
        }
        
        count
    }
}
