#![feature(test)]

extern crate test;

pub mod sorting {

    ///Merge Sort implementation:
    pub fn merge_sort(mut A: Box<Vec<i32>>, p: usize, r: usize) -> Box<Vec<i32>> {
        let q = (p + r) / 2;
        // If q and p are equal we end up making the same call as the parent call
        // resulting in a never ending loop creating a stack overflow
        if p < r && q != p {
            A = merge_sort(A, p, q);
            A = merge_sort(A, q, r);
            A = merge(A, p, q, r);
        }
        A
    }

    /// Partition an array returning the index of partiton
    fn merge(mut A: Box<Vec<i32>>, p: usize, q: usize, r: usize) -> Box<Vec<i32>> {
        // Build out the arrays
        let n_l = q - p;
        let n_r = r - q;
        let mut L: Vec<i32> = vec![];
        let mut R: Vec<i32> = vec![];

        for i in p..(p + n_l) {
            L.push(A[i]);
        }
        for j in q..(q + n_r) {
            R.push(A[j]);
        }

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
        A
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;
        use test::Bencher;

        #[rstest]
        fn test_merge_sort() {
            let A = vec![3, -1, 2, 0, 6, 1, -3, -4];
            let end = A.len();
            let A = merge_sort(Box::new(A), 0, end).to_vec();
            assert_eq!(*A, vec![-4, -3, -1, 0, 1, 2, 3, 6]);
        }

        #[bench]
        fn benchmark_merge_sort(b: &mut Bencher) {
            b.iter(|| {
                let A = vec![3, -1, 2, 0, 6, 1, -3, -4];
                let end = A.len();
                merge_sort(Box::new(A), 0, end).to_vec();
            });
        }
    }
}
