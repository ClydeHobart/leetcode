pub struct Solution;

impl Solution {
    /// Construct a 2D array from a 1D array with the given dimensions.
    ///
    /// # Retrospective
    ///
    /// Of the two public solutions at the time of writing this, one uses `[i32]::chunks`, which is
    /// cleaner than the `Range` business I used. It does run 2ms slower than mine, though. The
    /// other claims to be "100%" (not sure if this is allegedly speed, memory, or both). It uses
    /// `Vec::with_capacity`. I was under the impression that the implementation of `FromIterator`
    /// for `Vec` used size hinting to only allocate once, so I'm not sure if this provides a
    /// tangible benefit over my implementation or not.
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n): (usize, usize) = (m as usize, n as usize);

        if original.len() != m * n {
            Vec::new()
        } else {
            (0_usize..m)
                .into_iter()
                .map(|row| {
                    let start: usize = row * n;

                    original[start..start + n].iter().copied().collect()
                })
                .collect()
        }
    }
}

mod tests {
    use super::Solution;

    macro_rules! vec_2d {
        [ $( [ $( $e:expr ),* ] ),* ] => {
            vec![ $( vec![ $( $e, )* ], )* ]
        };
    }

    /// Example 1 from the problem description
    #[test]
    fn example_1() {
        let original: Vec<i32> = vec![1, 2, 3, 4];
        let m: i32 = 2_i32;
        let n: i32 = 2_i32;

        assert_eq!(
            Solution::construct2_d_array(original, m, n),
            vec_2d![[1, 2], [3, 4]]
        );
    }

    /// Example 2 from the problem description
    #[test]
    fn example_2() {
        let original: Vec<i32> = vec![1, 2, 3];
        let m: i32 = 1_i32;
        let n: i32 = 3_i32;

        assert_eq!(
            Solution::construct2_d_array(original, m, n),
            vec_2d![[1, 2, 3]]
        );
    }

    /// Example 3 from the problem description
    #[test]
    fn example_3() {
        let original: Vec<i32> = vec![1, 2];
        let m: i32 = 1_i32;
        let n: i32 = 1_i32;

        assert_eq!(
            Solution::construct2_d_array(original, m, n),
            Vec::<Vec<i32>>::new()
        );
    }
}
