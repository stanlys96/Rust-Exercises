fn main() {
    let mut array = [1, 2, 3, 4];
    // 1 - Use reverse() method
    array.reverse();
    println!("{:?}", array);
    
    // 2 - Create a new array and populate from the original array backwards.
    const array2: [i32; 4] = [4, 10, 42, 13];
    let mut array_reversed = [0; array2.len()];
    
    for i in 0..array2.len() {
        array_reversed[i] = array2[array2.len() - i - 1];
    }
    println!("{:?}", array_reversed);
    
    // 3 - Swap the places from early to end in the array.
    let mut array3 = [1, 2, 3, 4, 5, 199, 344];
    for i in 0..array3.len() / 2 {
        let j = array3.len() - i - 1;
        let temp = array3[i];
        array3[i] = array3[j];
        array3[j] = temp;
    }
    println!("{:?}", array3);
  }