use std::collections::HashMap;

// Brute force approach, check each element with the next elements, avoiding checking an element with itself.
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num1) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().skip(i + 1).enumerate() {
            if num1 + num2 == target {
                return vec![i as i32, (i + j + 1) as i32];
            }
        }
    }
    vec![]
}

// Use Hashmap, more performance friendly
fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let compliment = target - num;
        if let Some(&index) = hm.get(&compliment) {
            return vec![index as i32, i as i32];
        }
        hm.insert(num, i);
    }
    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));
    println!("{:?}", two_sum(vec![2, 7, 11, 15,], 9)); 
    println!("{:?}", two_sum(vec![3, 2, 4,], 6));
    println!("{:?}", two_sum(vec![3, 3,], 6));

    println!("{:?}", two_sum_hashmap(vec![2, 3, 4, 5,], 9));
    println!("{:?}", two_sum_hashmap(vec![2, 7, 11, 15,], 9)); 
    println!("{:?}", two_sum_hashmap(vec![3, 2, 4,], 6));
    println!("{:?}", two_sum_hashmap(vec![3, 3,], 6));
}
