// Find the maximum subarray sum in Rust

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_ending_here = nums[0];
    let mut max_so_far = nums[0];

    for &num in nums.iter().skip(1) {
        max_ending_here = max_ending_here.max(num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
