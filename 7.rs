// Implement a function that returns the kth smallest element in a given array.

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn quick_select(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if low == high {
        return arr[low];
    }

    let pivot_index = partition(arr, low, high);

    if pivot_index == k {
        return arr[pivot_index];
    } else if k < pivot_index {
        return quick_select(arr, low, pivot_index - 1, k);
    } else {
        return quick_select(arr, pivot_index + 1, high, k);
    }
}

fn find_kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        Some(quick_select(arr, 0, arr.len() - 1, k - 1))
    } else {
        None
    }
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    if let Some(kth_smallest) = find_kth_smallest(&mut arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k");
    }
}
