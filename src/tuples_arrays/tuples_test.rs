pub fn tuples_test() {
    let tuple = (1, 2, 3, 4, 5);
    let (a, b, c, d, e) = tuple;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
    println!("tuple.0 = {}", tuple.0);
    println!("tuple.1 = {}", tuple.1);
    println!("tuple.2 = {}", tuple.2);
    println!("tuple.3 = {}", tuple.3);
    println!("tuple.4 = {}", tuple.4);
}