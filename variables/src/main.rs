// 2023 01 24 00 29 AM

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 3 / 5;
    println!("The value of y is: {}", y);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    println!("The third value of the tuple is: {}", tup.2);
    let value_of_hello = print_hello(5);
    println!("The value of hello is: {}", value_of_hello);

    let x = 3;
    let y = {
        let x = 4;
        x + 1
    };
    println!("The value of x is {} and value of y is {}", x, y);

    let z = if 3 + 5 == 8 {
        2
    } else {
        33
    };

    println!("The value of number is : {}", z);

    let a = [1, 2, 3, 4, 5];
    for elem in a.iter() {
        println!("The value is: {}", elem);
    }
    
    for a in (1..=9).rev() {
        for b in (1..=9).rev() {
            print!("{} x {} = {}", a, b, a * b);
        }
        println!("");
    }

    println!("500F is {} ", f_to_c(500.0));
    println!("Fibo 13 is {} ", fibo(13)); // 233

    let presents = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in (0..12) {
        println!("On the {} day of Christmas my true love gave me {}", day + 1, presents[day]);
    }
}

fn print_hello(x: i32) -> i32 {
    println!("Hello world! {}", x);
    x % 2
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn fibo(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n-2)
    }
}
