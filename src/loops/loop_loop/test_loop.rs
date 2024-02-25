pub fn test_loop() {
    let mut i = 0;
    loop {
        i += 1;
        println!("i: {}", i);
        if i > 10 {
            break;
        }
    }
}
