pub mod arithmetic;
use crate::arithmetic::interproduct::interproduct;

pub mod string_test;
use crate::string_test::string_test::string_test;

pub mod fibonacci;
use crate::fibonacci::fibonacci::fibonacci;

pub mod condition;
use crate::condition::condition::check_condition;

pub mod loops;
use crate::loops::for_loop::test_for::test_for;
use crate::loops::loop_loop::test_loop::test_loop;
use crate::loops::while_loop::test_while::test_while;

fn main() {
    println!("========================================");
    println!("Rust Practices - Sabik Rahat");

    println!("========================================");
    println!("1. Arithmetic");
    let a = interproduct(2, 3, 4);
    println!("Interproduct: {}", a);

    println!("========================================");
    println!("2. String Test");
    let b = string_test();
    println!("String Test: {}", b);

    println!("========================================");
    println!("3. Fibonacci");
    let c = fibonacci(10);
    println!("Fibonacci: {}", c);

    println!("========================================");
    println!("4. Condition");
    let d = check_condition(10);
    println!("Condition: {}", d);
    let x = 20;
    let size = if x < 10 {
        "less than 10"
    } else if x > 10 {
        "greater than 10"
    } else {
        "equal to 10"
    };
    println!("Size: {}", size);

    println!("========================================");
    println!("5. Loops");
    test_while();
    test_loop();
    test_for();
}
