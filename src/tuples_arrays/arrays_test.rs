pub fn arrays_test() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);
    arr[0] = 10;
    println!("arr[0]: {}", arr[0]);
    println!("arr: {:?}", arr);
    println!("arr length: {}", arr.len());
    println!("arr occupies {} bytes", std::mem::size_of_val(&arr));
    let slice: &[i32] = &arr[1..3];
    println!("slice: {:?}", slice);

    let mut a: [i8; 10] = [42; 10];
    println!("a: {a:?}");
    a[5] = 0;
    println!("a: {a:?}");
}
