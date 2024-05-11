pub fn bin_k_t(k: u32, t: u32, current: usize, results: &mut Vec<usize>) {
    // Base case 1, we need to put all 1's in the remainder of the string.
    if k == t {
        let mask = usize::pow(2, k + 1) - 1;
        let current = current | mask;

        results.push(current);
    } else if t == 0 {
        // Base case 2,
        results.push(current);
    } else {
        let mask = usize::pow(2, k);
        bin_k_t(k - 1, t, current, results);
        bin_k_t(k - 1, t - 1, current | mask, results);
    }
}