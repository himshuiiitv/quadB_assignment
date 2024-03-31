// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        
        if arr[mid] == target {
            
            if mid == 0 || arr[mid - 1] < target {
                return Some(mid);
            } else {
                
                right = mid - 1;
            }
        } else if arr[mid] < target {
            
            left = mid + 1;
        } else {
            
            right = mid - 1;
        }
    }
    
    None 
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 3, 4, 5, 6];
    let target = 3;
    
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} is not found in the array", target),
    }
}
