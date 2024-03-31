// Merge two sorted arrays in Rust

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    let arr1 = [1, 3, 5, 7, 9];
    let arr2 = [2, 4, 6, 8];

    let merged_array = merge_sorted_arrays(&arr1, &arr2);

    println!("Merged sorted array: {:?}", merged_array);
}
