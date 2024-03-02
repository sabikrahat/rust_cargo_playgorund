pub mod arithmetic;
use crate::arithmetic::interproduct::interproduct;

pub mod string_test;
use crate::string_test::string_test::string_test;

pub mod fibonacci;
use crate::fibonacci::fibonacci::fibonacci;

pub mod condition;
use crate::condition::condition::check_condition;

pub mod loops;
use crate::loops::break_continue::break_continue_test;
use crate::loops::for_loop::test_for::test_for;
use crate::loops::loop_loop::test_loop::test_loop;
use crate::loops::scops_shadowing::scops_shadowing_test;
use crate::loops::while_loop::test_while::test_while;

pub mod collatz_sentence;
use crate::collatz_sentence::collatz_sentence::collatz_sentence_test;

pub mod tuples_arrays;
use crate::tuples_arrays::arrays_test::arrays_test;
use crate::tuples_arrays::nested_arrays::nested_arrays_transpose;
use crate::tuples_arrays::tuples_test::tuples_test;

pub mod reference;
use crate::reference::shared_reference::shared_reference_test;
use crate::reference::exclusive_reference::exclusive_reference_test;

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
    break_continue_test();
    scops_shadowing_test();

    println!("========================================");
    println!("6. Collatz Sentence");
    let n = collatz_sentence_test(11);
    println!("Collatz Sentence: {}", n);

    println!("========================================");
    println!("7. Arrays");
    arrays_test();

    println!("========================================");
    println!("8. Tuples");
    tuples_test();

    println!("========================================");
    println!("9. Nested Arrays");
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    let r = nested_arrays_transpose(matrix);
    println!("Nested Arrays: {:#?}", r);

    println!("========================================");
    println!("10. Reference");
    shared_reference_test();
    exclusive_reference_test();
}
