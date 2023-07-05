pub mod commands;
pub mod delta_patch;

fn main() {
    println!("Hello, world!");
        let source_bytes = vec![0, 1];
        let target_bytes = vec![0, 1];
        let mut source_byte_iter = source_bytes.iter().peekable();
        let mut target_bytes_iter = target_bytes.iter().peekable();
        let mut zip = source_byte_iter.by_ref().zip(target_bytes_iter.by_ref());
        zip.next();
        dbg!(source_byte_iter.next());
}
