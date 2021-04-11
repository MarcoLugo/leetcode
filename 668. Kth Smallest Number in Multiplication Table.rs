/*
Nearly every one have used the Multiplication Table. But could you find out
the k-th smallest number quickly from the multiplication table?

Given the height m and the length n of a m * n Multiplication Table, and a
positive integer k, you need to return the k-th smallest number in this table.

Note:

    The m and n will be in the range [1, 30000].
    The k will be in the range [1, m * n]
*/

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut low = 1;
        let mut high = m * n;

        while low < high {
            let mid = (low + high) / 2;
            let n_below_mid = (1..=m).fold(0, |acc, row_i| acc + n.min(mid / row_i));

            match n_below_mid.cmp(&k) {
                std::cmp::Ordering::Less => low = mid + 1,
                _ => high = mid,
            }
        }

        low
    }
}

