use std::collections::HashMap;

fn main() {
    let input = vec![1, 2, 3, 4, 5, 5];
    let (median, mode) = get_median_and_mode(input);
    println!("Median: {}, Mode: {}", median, mode);
}

fn get_median_and_mode(input: Vec<i32>) -> (f64, i32) {
    let mut input_sorted = input.clone();
    input_sorted.sort();

    let input_len = input_sorted.len();
    let median: f64;
    if input_len % 2 == 0 {
        let mid_high = input_sorted[input_len / 2] as f64;
        let mid_low = input_sorted[input_len / 2 - 1] as f64;
        median = (mid_high + mid_low) / 2.0;
    } else {
        median = input_sorted[input_len / 2] as f64;
    }

    let mut mode_set: HashMap<i32, i32> = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;
    for i in input {
        let count_i = mode_set.entry(i).or_insert(0);
        *count_i += 1;
        if *count_i > max_count {
            max_count = *count_i;
            mode = i;
        }
    }

    (median, mode)
}
