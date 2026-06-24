fn main() {
    let a = 67;
    let b = 69;
    let c = 42;
    let d = 52;
    let e = 1488;

    println!("{:-^10} | {:-^10} | {:-^10}", "Normal", "Ultra-Sigma", "Larp");
    println!("{:^10} | {:^10} | {:^10}", a, a*67, a*2);
    println!("{:^10} | {:^10} | {:^10}", b, b*67, b*4);
    println!("{:^10} | {:^10} | {:^10}", c, c*67, c*6);
    println!("{:^10} | {:^10} | {:^10}", d, d*67, d*8);
    println!("{:^10} | {:^10} | {:^10}", e, e*67, e*10);

    loop {}
}
