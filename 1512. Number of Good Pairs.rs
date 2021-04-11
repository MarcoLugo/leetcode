/*
Given an array of integers nums.

A pair (i,j) is called good if nums[i] == nums[j] and i < j.

Return the number of good pairs.

Constraints:

    1 <= nums.length <= 100
    1 <= nums[i] <= 100

*/


impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0i32;
        let mut counts = [0u8; 101];

        for num in nums {
            counts[num as usize] += 1;
        }

        for &amount in counts.iter() {
            count += i32::from(amount) * i32::from(amount - 1) / 2;
        }

        count
    }
}
