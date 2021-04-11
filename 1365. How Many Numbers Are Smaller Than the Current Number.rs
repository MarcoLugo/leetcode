/*
Given the array nums, for each nums[i] find out how many numbers in the array
are smaller than it. That is, for each nums[i] you have to count the number of
valid j's such that j != i and nums[j] < nums[i].

Return the answer in an array.

Constraints:

    2 <= nums.length <= 500
    0 <= nums[i] <= 100
*/

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut all_counts = [0i32; 101];
        let mut counts = HashMap::new();

        for num in &nums {
            all_counts[*num as usize] += 1;
        }

        let mut rolling_sum = 0;
        for (num, &num_count) in all_counts.iter().enumerate() {
            if num_count > 0 {
                counts.insert(num as i32, rolling_sum);
                rolling_sum += num_count;
            }
        }

        nums.iter().map(|x| *counts.get(x).unwrap()).collect()
    }
}

