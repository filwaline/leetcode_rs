/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 * [704] 二分查找
 */

// @lc code=start

impl Solution {
    // pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    //     let (mut l, mut r) = (0, nums.len() - 1);

    //     while l < r {
    //         let mid = (l + r + 1) >> 1;

    //         if nums[mid] <= target {
    //             l = mid;
    //         } else {
    //             r = mid - 1;
    //         }
    //     }

    //     if nums[l] == target {
    //         l as i32
    //     } else {
    //         -1
    //     }
    // }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            let mid = (l + r) >> 1;

            if nums[mid] >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if nums[l] == target {
            l as i32
        } else {
            -1
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3],4 => -1)]
    #[test_case(vec![1,3,5,7,9], 6 => -1)]
    #[test_case(vec![1,3,5,7,9], 3 => 1)]
    #[test_case(vec![2,4,6,8], 2 => 0)]
    #[test_case(vec![2,4,6,8], 3 => -1)]
    #[test_case(vec![2,4,6,8], 9 => -1)]
    fn test_binary_search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search(nums, target)
    }
}
