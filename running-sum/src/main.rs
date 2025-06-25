// Use ordinary for loop
fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut number_sum = 0;
    let mut result = Vec::new();
    for i in nums {
        number_sum += i;
        result.push(number_sum);
    }
    result
}

// Use idiomatic Rust
fn running_sum_idiomatic(nums: Vec<i32>) -> Vec<i32> {
    let mut number_sum = 0;
    nums.iter()
        .map(|num| {
            number_sum += num;
            number_sum
        })
        .collect()
}

fn main() {
    println!("{:?}", running_sum(vec![1, 2, 3, 4]));
    println!("{:?}", running_sum(vec![3, 1, 2, 10, 1]));

    println!("{:?}", running_sum_idiomatic(vec![1, 2, 3, 4]));
    println!("{:?}", running_sum_idiomatic(vec![3, 1, 2, 10, 1]));
}
