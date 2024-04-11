fn main() {
    // create vector
    let v1 = vec![1, 2, 3];

    // create iterator to iterate over vector
    let v1_iter = v1.iter();

    // use iterator in loop
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // create iterator that takes ownership
    let v1_iter_own = v1.into_iter();

    let mut v1_mut = vec![1, 2, 3];

    // create iterator over mutable references
    let v1_iter_mut = v1_mut.iter_mut();

    // create iterator which returns a new iterator which modifies items from a vector
    let v2: Vec<i32> = vec![1, 2, 3];
    let v2_new: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v2_new, vec![2, 3, 4]);
}
