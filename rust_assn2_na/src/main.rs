fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let arry = [23, 25, 22, 17, 12, 7, 10, 13, 19, 32];

    for num in arry {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            println!("{} is even? {}", num, is_even(num));
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < arry.len() {
        sum += arry[i];
        i += 1;
    }
    println!("Sum of the numbers: {}", sum);

    let mut max = arry[0];
    for num in arry {
        if num > max {
            max = num;
        }
    }
    println!("Biggest number is: {}", max);
}
