use std::collections::HashMap;

fn main() {
    let input = vec![1, 2, 3, 4, 5, 5];
    let (median, mode) = calculate_statistics(&input);
    println!("Median: {}, Mode: {}", median, mode);
}

fn calculate_statistics(input: &[i32]) -> (f64, i32) {
    let median = calculate_median(input);
    let mode = calculate_mode(input);
    
    (median, mode)
}

fn calculate_median(input: &[i32]) -> f64 {
    let mut input_sorted = input.to_vec();
    input_sorted.sort();

    let input_len = input_sorted.len();
    if input_len % 2 == 0 {
        let mid_high = input_sorted[input_len / 2] as f64;
        let mid_low = input_sorted[input_len / 2 - 1] as f64;
        (mid_high + mid_low) / 2.0
    } else {
        input_sorted[input_len / 2] as f64
    }
}

fn calculate_mode(input: &[i32]) -> i32 {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;
    
    for &value in input {
        let count = frequency_map.entry(value).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = value;
        }
    }
    
    mode
}
