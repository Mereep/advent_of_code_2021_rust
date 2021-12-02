/// Straight-forward implementation which just compares two numbers
pub fn task1(data: &Vec<u64>) -> u64 {

    // counts how often we increase the number
    let mut up_count: u64 = 0;

    // saves the last number which we saw
    let mut last_num: Option<&u64> = None;

    // iterate over each number
    for n in data {

        // Check if we have a last number
        if let Some(before) = last_num {
            if n > before {
                up_count += 1;
            }
        }

        // save that last number
        last_num = Some(n);
    }

    up_count
}

/// Functional version
/// Example: `[1,2,3,4,5,6]`, `window_size = 3`
/// - builds overlapping windows of `window_size` elements -> `[[1,2,3], [2,3,4], ... [4,5,6]]`
/// - sums each window to a single number -> collapse those windows two their sum `[6,9, ..., 15]`
/// - builds again windows. Thi time of size two (i.e., two neighbouring pairs of collapsed windows)
///   `[[6,9], ..., [12, 15]]`
/// - folds those windows to a single increasing number if the second entry is bigger than the first
///   `9 > 6 ? + 1 + ... + 15 > 12 ? + 1`
pub fn task2_functional(data: &Vec<u64>, window_size: usize) -> u64 {
    let collapsed= data
        .windows(window_size)
        .map(|w| w.iter().sum())
        .collect::<Vec<u64>>()// idk if there is a way to not collect them but chain directly
        .windows(2)
        .fold::<u64, _>(0, |p, cw| p + ((cw[1] > cw[0]) as u64));

    return collapsed;
}

/// Generic function iterating in chunks of size `window_size + 1`
/// (@see https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
/// over the data lines (slower implementation)
pub fn task2_slow(data: Vec<u64>, window_size: usize) -> u64 {
    let mut up_count: u64 = 0;

    for i in data.as_slice().windows(window_size + 1) {

        // This is slow, we could do this better by just remembering a sum and removing the last entry
        let sum: u64 = i.iter().sum();
        let a = sum - i[window_size];
        let b = sum - i[0];
        if b > a {
            up_count += 1;
        }

        // println!("{:?}, {}:{}", i, a, b);
    }

    return up_count;
}


/// Generic and somewhat faster version
pub fn task2(data: Vec<u64>, window_size: usize) -> u64 {

    // we interpret the data as an array
    // so that indexed access (should) be faster than an array
    // Rust may optimize the vector versions, too, though
    let data_as_array = data.as_slice();
    let mut up_count: u64 = 0;
    let mut a: u64 = data_as_array[0..window_size].iter().sum();

    for i in window_size..data.len() {
        // This is slow, we could do this better by just remembering a sum and removing the last entry
        let b = a - data_as_array[i - window_size] + data_as_array[i];
        if b > a {
            up_count += 1;
        }
        a = b;
    }

    return up_count; // 1627
}
