pub fn binary_search<T: Ord>(input: &[T], element: T) -> usize {
    let mut i = 0;
    let mut j = input.len();
    let mut k = j / 2;

    while (input[k] != element) && (i <= j) {
        k = (i + j) / 2;
        if element > input[k] {
            i = k + 1;
        } else if k > 0 {
            j = k - 1;
        }
    }

    k
}
