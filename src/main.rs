use clrs::MergeSorter;

fn main() {
    let mut A = vec![1, -1, -2, 8, 3, 6, 5];

    dbg!(&A);
    MergeSorter::sort(&mut A);
    dbg!(&A);
}
