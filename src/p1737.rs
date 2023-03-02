/// # 1737. Change Minimum Characters to Satisfy One of Three Conditions
///
/// [Link][link]
///
/// [link]: https://leetcode.com/problems/change-minimum-characters-to-satisfy-one-of-three-conditions/
pub struct Solution;

use std::slice::SliceIndex;

#[derive(Default)]
struct Counts {
    counts: [i32; 26_usize],
    total: i32,
}

impl Counts {
    fn increment(&mut self, string: &str) {
        for byte in string.as_bytes().iter().copied() {
            self.counts[(byte - b'a') as usize] += 1_i32;
        }

        self.total = string.len() as i32;
    }

    fn sum_gt(&mut self, other: &Self) {
        self.counts
            .iter_mut()
            .zip(other.counts.iter().copied())
            .rev()
            .fold(0_i32, |sum, (self_sum, other_count)| {
                *self_sum = sum;

                sum + other_count
            });
    }

    fn sum_le(&mut self, other: &Self) {
        self.counts
            .iter_mut()
            .zip(other.counts.iter().copied())
            .fold(0_i32, |mut sum, (self_sum, other_count)| {
                sum += other_count;
                *self_sum = sum;

                sum
            });
    }

    fn sum_iter<'a>(
        &'a self,
        other: &'a Self,
        range: impl SliceIndex<[i32], Output = [i32]> + Clone,
    ) -> impl Iterator<Item = i32> + 'a {
        self.counts[range.clone()]
            .iter()
            .copied()
            .zip(other.counts[range].iter().copied())
            .map(|(a, b)| a + b)
    }

    fn total(&self) -> i32 {
        self.total
    }
}

impl From<&str> for Counts {
    fn from(value: &str) -> Self {
        let mut counts: Self = Self::default();

        counts.increment(value);

        counts
    }
}

trait SolutionTrait<T: ?Sized> {
    fn min_characters_condition_1(a: &T, b: &T) -> i32;

    fn min_characters_condition_2(a: &T, b: &T) -> i32 {
        Self::min_characters_condition_1(b, a)
    }

    fn min_characters_condition_3(a: &T, b: &T) -> i32;
}

impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut a_counts: Counts = Counts::default();
        let mut b_counts: Counts = Counts::default();

        a_counts.increment(&a);
        b_counts.increment(&b);

        [
            Self::min_characters_condition_1(&a_counts, &b_counts),
            Self::min_characters_condition_2(&a_counts, &b_counts),
            Self::min_characters_condition_3(&a_counts, &b_counts),
        ]
        .iter()
        .copied()
        .min()
        .unwrap()
    }
}

impl SolutionTrait<str> for Solution {
    fn min_characters_condition_1(a: &str, b: &str) -> i32 {
        Self::min_characters_condition_1(&Counts::from(a), &Counts::from(b))
    }

    fn min_characters_condition_3(a: &str, b: &str) -> i32 {
        Self::min_characters_condition_3(&Counts::from(a), &Counts::from(b))
    }
}

impl SolutionTrait<Counts> for Solution {
    fn min_characters_condition_1(a: &Counts, b: &Counts) -> i32 {
        let mut a_sum_gt: Counts = Counts::default();
        let mut b_sum_le: Counts = Counts::default();

        a_sum_gt.sum_gt(&a);
        b_sum_le.sum_le(&b);

        a_sum_gt.sum_iter(&b_sum_le, ..25_usize).min().unwrap()
    }

    fn min_characters_condition_3(a: &Counts, b: &Counts) -> i32 {
        a.total() + b.total() - a.sum_iter(b, ..).max().unwrap()
    }
}

mod tests {
    use super::{Solution, SolutionTrait};

    const EXAMPLE_1_A: &str = "aba";
    const EXAMPLE_1_B: &str = "caa";
    const EXAMPLE_2_A: &str = "dabadd";
    const EXAMPLE_2_B: &str = "cda";
    const EXAMPLE_3_A: &str = "aaa";
    const EXAMPLE_3_B: &str = "zzz";

    #[test]
    fn example_1_condition_1() {
        assert_eq!(
            Solution::min_characters_condition_1(EXAMPLE_1_A, EXAMPLE_1_B),
            2_i32
        );
    }

    #[test]
    fn example_1_condition_2() {
        assert_eq!(
            Solution::min_characters_condition_2(EXAMPLE_1_A, EXAMPLE_1_B),
            3_i32
        );
    }

    #[test]
    fn example_1_condition_3() {
        assert_eq!(
            Solution::min_characters_condition_3(EXAMPLE_1_A, EXAMPLE_1_B),
            2_i32
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_characters(EXAMPLE_1_A.into(), EXAMPLE_1_B.into()),
            2_i32
        );
    }

    #[test]
    fn example_2_condition_1() {
        assert_eq!(
            Solution::min_characters_condition_1(EXAMPLE_2_A, EXAMPLE_2_B),
            3_i32
        );
    }

    #[test]
    fn example_2_condition_2() {
        assert_eq!(
            Solution::min_characters_condition_2(EXAMPLE_2_A, EXAMPLE_2_B),
            4_i32
        );
    }

    #[test]
    fn example_2_condition_3() {
        assert_eq!(
            Solution::min_characters_condition_3(EXAMPLE_2_A, EXAMPLE_2_B),
            5_i32
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_characters(EXAMPLE_2_A.into(), EXAMPLE_2_B.into()),
            3_i32
        );
    }

    #[test]
    fn example_3_condition_1() {
        assert_eq!(
            Solution::min_characters_condition_1(EXAMPLE_3_A, EXAMPLE_3_B),
            0_i32
        );
    }

    #[test]
    fn example_3_condition_2() {
        assert_eq!(
            Solution::min_characters_condition_2(EXAMPLE_3_A, EXAMPLE_3_B),
            6_i32
        );
    }

    #[test]
    fn example_3_condition_3() {
        assert_eq!(
            Solution::min_characters_condition_3(EXAMPLE_3_A, EXAMPLE_3_B),
            3_i32
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::min_characters(EXAMPLE_3_A.into(), EXAMPLE_3_B.into()),
            0_i32
        );
    }
}
