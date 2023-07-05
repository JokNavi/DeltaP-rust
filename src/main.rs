pub mod instructions;
pub mod delta_p;

fn main() {
    println!("Hello, world!");
    let mut vector: Vec<i32> = vec![0;10];
    dbg!(vector.get(11..));
    dbg!('+' as u8);
    dbg!('-' as u8);
    dbg!('C' as u8);
    dbg!('R' as u8);
    dbg!('&' as u8);
}
