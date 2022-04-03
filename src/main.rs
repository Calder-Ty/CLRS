fn main() {
    let mut A = Box::new(vec![1, -1, -2, 8, 3, 6, 5]);
    let len = A.len();

    dbg!(&A);
    A = merge_sort(A, 0, len);
    dbg!(&A);
}

///Merge Sort implementation:
fn merge_sort(mut A: Box<Vec<i32>>, p: usize, r: usize) -> Box<Vec<i32>> {
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
