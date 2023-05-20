use std::time::{SystemTime, UNIX_EPOCH};
/// The randomization lcg_random module represents a linear congruence generator.
/// A linear congruence generator can be used to generate pseudo random number of 
/// somewhat even distribution. This can be useful in testing different algorithms
/// on random input. Or even for scrambling lists such as when preparing for quick sort
/// 
/// This implementation uses the GLIBC (GNU C Library)'s parameters for the LCG,
/// and seeds it using the time. The recurrence relation being:
/// `x = (1103515245 * x + 12345) % 2147483648`.
/// 
/// Do note that this implementation is *NOT* cryptograhpically secure.
/// 
/// Author: cave
/// 
/// # Examples
/// ```
/// use itualgs_rs::randomization::lcg_random::lcg_generate;
/// 
/// let random_vector: Vec<u32> = lcg_generate(200);
/// 
/// assert!(random_vector.len() == 200);
/// ``` 
pub fn lcg_generate(size: usize) -> Vec<u32> {
    let mut random_list = Vec::new();

    // Seed the linear congruence generator with
    // the current system time. This is not
    // cryptographically secure. However we
    // don't need this.
    let mut x = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64;

    // Run the linear congruence generator
    for _ in 0..size {
        x = (1103515245 * x + 12345) % 2147483648 as u64;
        random_list.push(x as u32);
    }
    
    random_list
}

#[cfg(test)]
mod tests {
    use super::lcg_generate;

    #[test]
    fn test_size() {
        let size = 2000;
        let arr = lcg_generate(size);
        assert!(arr.len() == size);
    }

    #[test]
    fn test_uniform() {
        let size = 200_000;
        let arr = lcg_generate(size);

        // Assert the length of generated array
        assert_eq!(arr.len(), size);

        let num_bins = 100;
        // Gives ten percentage tolerance
        let tolerance_factor = 10;

        // Initialize the counter for each bin
        let mut bins = vec![0; num_bins];

        // For each number, increment the count for its bin
        for &element in &arr {
            let bin_index = (element % num_bins as u32) as usize;
            bins[bin_index] += 1;
        }

        // Calculate average and tolerance
        let avg_count = size / num_bins;
        let tolerance = avg_count / tolerance_factor;

        // Assert that each bin's count is within the tolerance
        for count in bins {
            assert!(count >= avg_count - tolerance, "Count was too low: {}", count);
            assert!(count <= avg_count + tolerance, "Count was too high: {}", count);
        }
    }
}
