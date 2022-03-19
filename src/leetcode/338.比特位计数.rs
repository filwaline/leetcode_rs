/*
 * @lc app=leetcode.cn id=338 lang=rust
 *
 * [338] 比特位计数
 */

// @lc code=start
impl Solution {
    // pub fn count_bits(n: i32) -> Vec<i32> {
    //     let n = n as usize;
    //     let mut result = Vec::with_capacity(n + 1);

    //     result.push(0);

    //     while result.len() <= n {
    //         let r: Vec<i32> = result.iter().map(|&i| i + 1).collect();
    //         result.extend_from_slice(&r);
    //     }

    //     (&result[..n + 1]).to_vec()
    // }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; n + 1];
        let mut b = 1;
        for i in 1..=n {
            if i >= 2 * b {
                b *= 2;
            }
            result[i] = result[i - b] + 1;
        }
        result
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_5() {
        assert!(Solution::count_bits(5) == vec![0, 1, 1, 2, 1, 2])
    }

    #[test]
    fn test_2() {
        assert!(Solution::count_bits(2) == vec![0, 1, 1])
    }

    #[test_case(3 => vec![0,1,1,2]; "Input 3")]
    #[test_case(6 => vec![0,1,1,2,1,2,2]; "Input 6")]
    fn test_n(n: i32) -> Vec<i32> {
        Solution::count_bits(n)
    }
}
