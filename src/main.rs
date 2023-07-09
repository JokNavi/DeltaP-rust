use crate::{delta_patch::Patch, commands::command_util::ToBytes};

pub mod commands;
pub mod delta_patch;

fn main() {
    println!("Hello, world!");
        let source_bytes = b"My white cat likes jumping over the fence.";
        let target_bytes = b"My brown cat likes jumping over the stall.";
        let delta = Patch::encode(source_bytes, target_bytes);
        println!("{}", delta.to_bytes().iter().map(|byte| *byte as char).collect::<String>());
}
