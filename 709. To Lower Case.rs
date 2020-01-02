// Implement function ToLowerCase() that has a string parameter str, and returns the same string in lowercase.

impl Solution {
    pub fn to_lower_case(str: String) -> String {  
        let mut buffer = vec![];
        
        for c in str.as_bytes().into_iter() {
            if (65 <= *c) && (*c <= 90) {
                buffer.push(c + 32);
            } else {
                buffer.push(*c);
            }
        }

    String::from_utf8(buffer).expect("Found invalid UTF-8")
    }
}
