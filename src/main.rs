fn main() {
    // TODO move to separate files, cli specifies which solution to run
    solution1();
    solution2();
}

fn solution1() {
    // https://projecteuler.net/problem=1
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

fn solution2() {
    // https://projecteuler.net/problem=2
    // Find sum of all even fibonacci numbers <= 4,000,000
    // (Doesn't matter for this particular exercise, but starts the sequence with 1 and 2)
    // See notes:
    // https://github.com/WaterGenie35/project-euler-with-rust/wiki/Problem-2-%E2%80%90-Even-Fibonacci-Numbers
    let mut sum = 2;
    let mut prev = 2;
    let mut head = 8;
    while head <= 4_000_000 {
        sum += head;
        let tmp = head;
        head = (4 * head) + prev;
        prev = tmp;
    }
    println!("Sum of even fibonacci <= 4,000,000: {sum}");
}
