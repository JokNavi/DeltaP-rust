pub mod commands;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec_with_limit;

fn roundtrip(data: &[u8]) {
    // Compress the input
    let compressed = compress_to_vec(data, 9);
    println!("{:?}", &compressed.iter().map(|x| *x as char).collect::<String>());
    dbg!(compressed.len());
    dbg!(data.len());
    // Decompress the compressed input and limit max output size to avoid going out of memory on large/malformed input.
    let decompressed = decompress_to_vec_with_limit(compressed.as_slice(), 60000).expect("Failed to decompress!");
    // Check roundtrip succeeded
    println!("{:?}", &decompressed.iter().map(|x| *x as char).collect::<String>());
    assert_eq!(data, decompressed);
}

fn main() {
    roundtrip(b"-4AAAA+4BBBB#3-8CCCCAAAA+8EEEEEEEE");
}

