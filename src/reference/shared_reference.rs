pub fn shared_reference_test(){
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("shared reference1: {}", *r);
    r = &b;
    println!("shared reference2: {}", *r);
}