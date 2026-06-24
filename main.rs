fn main() {
    let a:u8 = 67;
    let b:u8 = 69;
    let c:u8 = 42;
    let d:u8 = 52;
    let e:u16 = 1488;

    println!("{:-^10} | {:-^10} | {:-^10}", "Normal", "Ultra-Sigma", "Larp");
    println!("{:^10} | {:^10} | {:^10}", a, a as u16*67, a as u16*2);
    println!("{:^10} | {:^10} | {:^10}", b, b as u16*67, b as u16*4);
    println!("{:^10} | {:^10} | {:^10}", c, c as u16*67, c as u16*6);
    println!("{:^10} | {:^10} | {:^10}", d, d as u16*67, d as u16*8);
    println!("{:^10} | {:^10} | {:^10}", e, e as u32*67, e as u32*10);

    loop {}
}
