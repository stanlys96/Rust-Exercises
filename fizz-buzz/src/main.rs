fn main() {
    println!("Welcome to the Fizzbuzz game!");
    let mut counter = 0;
    // 1 - using a for loop and if statements
    for count in 0..=301 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizz buzz");
            counter += 1;
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
    }
    println!("fizz buzz was called {} times!", counter);

    // 2 - using a for loop and match
    let mut counter2 = 0;
    for count in 0..=301 {
        match(count % 3, count % 5) {
            (0, 0) => {
                println!("fizz buzz");
                counter2 += 1;
            }
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => {}
        }
    }
    println!("fizz buzz 2 was called {} times!", counter2);
}
