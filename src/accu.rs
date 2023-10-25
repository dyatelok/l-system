pub const fn accumulate<const N: usize>(terms: [f32; N]) -> [f32; N] {
    let mut sums = [0.0; N];
    let mut sum = 0.0;
    let mut i = 0;
    while i < N {
        sum += terms[i];
        if sum > 1.0 {
            panic!("probability overflow",);
        }
        sums[i] = sum;
        i += 1;
    }
    if sum < 1.0 {
        panic!("summ of the probabilities is smaller than 1.0");
    }
    sums
}
