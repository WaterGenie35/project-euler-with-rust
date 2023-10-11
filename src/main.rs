fn main() {
    solution1();
}

fn solution1() {
    // Find sum of multiples of 3 or 5 below 1,000
    let mut sum = 0;
    let mut counter = 1;
    while counter < 1_000 {
        if  counter % 3 == 0 || counter % 5 == 0 {
            sum += counter;
        }
        counter += 1;
    }
    println!("Sum of multiples of 3 or 5 below 1,000: {sum}");
}
}
