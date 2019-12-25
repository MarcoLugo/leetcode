// Given an integer number n, return the difference between the product of its digits and the sum of its digits.

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let s = n.to_string();
        let mut product = 1;
        let mut difference = 0;
        
        for d in s.chars() {
            let d = d.to_digit(10).unwrap() as i32; // shadow
            product *= d;
            difference += d;
        }
        
        product - difference
    }
}
