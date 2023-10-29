fn main() {
    // TODO move to separate files, cli specifies which solution to run
    solution_1();
    solution_2();
    solution_4();
}

fn solution_1() {
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

fn solution_2() {
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

fn solution_4() {
    // https://projecteuler.net/problem=4
    // Find the largest palindrome made from the product of two 3-digit numbers
    // TODO clean up the maths
    let mut term1 = 999;
    let mut term2 = 999;
    let mut smallest_term = 100;
    let mut largest_palindrome = 0;
    while term1 >= smallest_term {
        while term2 >= smallest_term {
            let product = term1 * term2;
            if is_palindrome(product) {
                if product > largest_palindrome {
                    largest_palindrome = product;
                }
                break;
            }
            term2 -= 1;
        }
        term1 -= 1;
        term2 = term1;
    }
    if largest_palindrome == 0 {
        println!("Couldn't find a palindrome");
    } else {
        println!("Largest palindrome from products of two 3-digit numbers: {largest_palindrome}");
    }
}

// TODO move to separate module thingy and add tests
fn is_palindrome(number: i32) -> bool {
    let s = number.to_string();
    let b = s.as_bytes();
    let half_length = b.len() / 2;
    for i in 0..half_length {
        let j = b.len() - i - 1;
        if b[i] != b[j] {
            return false;
        }
    }
    return true;
}
