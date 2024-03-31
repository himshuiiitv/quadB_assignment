// Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 1 {
        
        arr[len / 2] as f64
    } else {
        
        ((arr[len / 2 - 1] + arr[len / 2]) as f64) / 2.0
    }
}

fn main() {
    let arr_odd = [1, 3, 5, 7, 9];
    let arr_even = [2, 4, 6, 8];

    println!("Median of arr_odd: {}", find_median(&arr_odd));
    println!("Median of arr_even: {}", find_median(&arr_even));
}
