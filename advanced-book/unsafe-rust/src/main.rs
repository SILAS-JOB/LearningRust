unsafe  fn split_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    (&mut values[..mid], &mut values[mid..])
}

fn main() {
    let mut vector = vec![1,2,3,4,5,6];
    let (left, right)  = split_mut(&mut vector, 3);
}
