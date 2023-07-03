### `Commands`:
```rust
struct Copy(u32, u8), //Contains the chunk starting index in the source file and the chunk's length.
struct Add(Vec<u8>, u8), //Contains the new bytes that need to be added and it's length.
struct Reference(u16); //Contains the index of a PatchCommand object with the same content. Can hold untill index 65535.
```

### `Save structure`:
```rust
// Patch_1 {
// Source = AAABBBAAACCC
// target = AAAXXXCCCDDD
// }
let commands = vec![Copy{3, 0000}, Add{3, "XXX"}, Reference{00}, Add{6, "CCCDDD"}]
//Final result = "|30000+3XXX&00+CCCDDD"
```


TEST 1:

Patch_1 {
Source = AAABBBAAACCC
target = AAAXXXCCCDDD
}

Patch_1 {
Source = AAAXXXCCCDDD
target = XXXBBBCCCXXX
}