### `Commands`:
```rust
struct CopyCommand(u8), //Contains the chunk's length.
struct AddCommand(u8, Vec<u8>), //Contains the new bytes that need to be added and it's length.
struct RemoveCommand(u8, Vec<u8>), //Contains the new bytes that need to be added and it's length.
struct ReferenceCommand(u16); //Contains the index of a PatchCommand object with the same content. Can hold untill index 65535.
```


## `Save structure`:
```rust
// Roses are red, violets are blue,\n\I wrote this library here,\n\just for you.\n\(It's true). 
// =>
// Roses are red, violets are blue,\n\I wrote this documentation here,\n\just for you.\n\(It's quite true).
//
// bytes = b""

let commands = vec[
    Same("Roses are red, violets are blue,I wrote this "),
    Rem("l"),
    Add("documentat"),
    Same("i"),
    Rem("brary"),
    Add("on"),
    Same(" here,just for you.(It's "),
    Add("qui"),
    Same("t"),
    Add("e t"),
    Same("rue)."),
]
```