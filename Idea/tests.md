### `Commands`:
```rust
struct Copy(u8), //Contains the chunk's length and the index at which it occured.
struct Add(u8, Vec<u8>), //Contains the new bytes that need to be added and it's length.
struct Remove(u8, Vec<u8>), //Contains the new bytes that need to be added and it's length.
struct Reference(u16); //Contains the index of a PatchCommand object with the same content. Can hold untill index 65535.
```


## `Save structure`:
```rust
// AAABBBAAABBB => AAAXXXAAADDD
let commands = vec![Copy{3}, Remove{3, b"BBB"}, Add{3, b"XXX"}, Copy{3}, Reference{2}, Add{3, b"DDD"}]
```




