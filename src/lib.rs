pub use sorting::{BubbleSorter, MergeSorter};

mod sorting {
    pub trait MergeSorter {
        fn sort(&mut self);
    }

    impl MergeSorter for Vec<i32> {
        fn sort(&mut self) {
            let len = self.len();
            merge_sort(self, 0, len);
        }
    }

    pub trait BubbleSorter {
        fn sort(&mut self);
    }

    impl BubbleSorter for Vec<i32> {
        fn sort(&mut self) {
            bubble_sort(self);
        }
    }

    ///Merge Sort implementation:
    fn merge_sort(A: &mut Vec<i32>, p: usize, r: usize) {
        let q = (p + r) / 2;
        // If q and p are equal we end up making the same call as the parent call
        // resulting in a never ending loop creating a stack overflow
        if p < r && q != p {
            merge_sort(A, p, q);
            merge_sort(A, q, r);
            merge(A, p, q, r);
        }
    }

    /// Partition an array returning the index of partiton
    fn merge(A: &mut Vec<i32>, p: usize, q: usize, r: usize) {
        // Build out the arrays
        let n_l = q - p;
        let n_r = r - q;
        let mut L: Vec<i32> = A.iter().skip(p).take(n_l).map(|x| *x).collect();
        let mut R: Vec<i32> = A.iter().skip(q).take(n_r).map(|x| *x).collect();
        L.push(std::i32::MAX);
        R.push(std::i32::MAX);

        let mut i = 0;
        let mut j = 0;

        for k in p..r {
            if L[i] <= R[j] {
                A[k] = L[i];
                i += 1;
            } else {
                A[k] = R[j];
                j += 1;
            }
        }
    }

    fn bubble_sort(arr: &mut Vec<i32>) {
        for i in 0..(arr.len() - 1) {
            for j in ((i + 1)..arr.len()).rev() {
                if arr[j] < arr[j - 1] {
                    arr.swap(j, j - 1);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::{fixture, rstest};

        #[fixture]
        fn unsorted() -> Vec<i32> {
            vec![
                -85, -11, -10, -9, -8, -7, -6, -5, -4, -3, -69, -63, -59, -45, -44, -40, -39, -38,
                13, 66, 94, 95, 96, 97, 98, 99, 100, -2, -1, 0, 1, 2, 31, 32, 33, 34, 35, 36, 37,
                -24, -23, -22, -21, 21, -15, -14, -13, -12, -100, -99, -98, -97, -96, -95, -94, 59,
                60, 61, 62, 3, 4, 5, 6, 7, -84, -83, -82, -81, -80, -68, -67, -66, -65, -64, 23,
                24, 25, 26, 27, 28, 29, 30, 67, 68, 69, 50, 51, 52, 53, 54, 8, 9, 10, 11, 12, -78,
                -77, -76, -75, -74, -73, -72, -58, -57, -56, -55, -54, -53, -47, -46, -37, 38, 39,
                40, 41, 84, 76, 77, 78, 79, 80, 81, 82, 83, 42, 43, 44, 45, 55, 56, 57, 58, 72, 73,
                74, 75, 46, 47, 48, 49, 14, 15, 16, 17, 18, 19, 20, 63, 64, 65, -52, -51, -50, -49,
                -48, -36, -35, -34, -33, -32, -20, -19, -18, -17, -16, -79, -31, -30, -29, -28,
                -27, -26, -25, 85, 86, 87, 88, 89, 90, 91, 92, 93, 22, 70, 71, -93, -92, -71, -70,
                -91, -90, -89, -62, -61, -60, -88, -87, -86, -43, -42, -41,
            ]
        }

        #[fixture]
        fn sorted() -> Vec<i32> {
            vec![
                -100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85,
                -84, -83, -82, -81, -80, -79, -78, -77, -76, -75, -74, -73, -72, -71, -70, -69,
                -68, -67, -66, -65, -64, -63, -62, -61, -60, -59, -58, -57, -56, -55, -54, -53,
                -52, -51, -50, -49, -48, -47, -46, -45, -44, -43, -42, -41, -40, -39, -38, -37,
                -36, -35, -34, -33, -32, -31, -30, -29, -28, -27, -26, -25, -24, -23, -22, -21,
                -20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3,
                -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
                63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
                84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
            ]
        }

        #[rstest]
        fn test_merge_sort(mut unsorted: Vec<i32>, sorted: Vec<i32>) {
            MergeSorter::sort(&mut unsorted);
            assert_eq!(unsorted, sorted);
        }

        #[rstest]
        fn test_bubble_sort(mut unsorted: Vec<i32>, sorted: Vec<i32>) {
            BubbleSorter::sort(&mut unsorted);
            assert_eq!(unsorted, sorted);
        }
    }
}
