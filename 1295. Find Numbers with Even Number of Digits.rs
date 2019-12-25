// Given an array nums of integers, return how many of them contain an even number of digits. 

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        
        for num in nums.into_iter() {
            let mut inner_count = 0;
            let mut number = num;
            
            loop {
                number /= 10;
                inner_count += 1;
                
                if number == 0 {
                    break;
                }
            }
            
            if inner_count % 2 == 0 {
                count += 1;
            }
        }
        
        count
    }
}