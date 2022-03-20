/*
 * @lc app=leetcode.cn id=1154 lang=rust
 *
 * [1154] 一年中的第几天
 */

// @lc code=start
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let ymd: Vec<usize> = date
            .split('-')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let [y, m, d] = [ymd[0], ymd[1], ymd[2]];

        let days = days_per_month.iter().enumerate().fold(
            0,
            |s, (i, &d)| {
                if i < m - 1 {
                    s + d
                } else {
                    s
                }
            },
        ) + (d as i32);

        match (y % 4 == 0, y % 100 == 0, y % 400 == 0, m > 2) {
            (true, false, _, true) => days + 1,
            (true, true, true, true) => days + 1,
            _ => days,
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("2019-01-09" => 9)]
    #[test_case("2008-03-01" => 61)]
    #[test_case("2004-02-03" => 34)]
    #[test_case("2000-12-04" => 339)]
    fn test_day_of_year(date: &str) -> i32 {
        Solution::day_of_year(date.to_string())
    }
}
