# getting-rusty

Just a place for all things Rust... for now

# Data Structures and Algorithms in Rust

### File Structure:

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── bst.rs
│   ├── dp.rs
│   ├── encoding.rs
│   ├── graphs.rs
│   ├── hashmaps.rs
│   ├── lib.rs
│   ├── linkedlists.rs
│   ├── rand_gen.rs
│   ├── sorting.rs
│   └── stacks.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

### Topics Included

- Binary Trees and Balanced Binary Trees
- Dynamic Programming (simple)
- Huffman Encoding
- Graphs and Graph Algorithms
- Hashmaps (self-implemented)
- LinkedLists and Doubly Linked Lists
- Pseudo Random Number Generator
- Sorting algorithms (Bubble, Merge and Quicksort) -- the latter has a thread-pool implementation for even faster performance
- Stacks data structure

### Running Tests from `lib.rs`

Run the following command:

```bash
cargo test <test_function_name> -- --nocapture # To print outputs
```

Or run the entire test module

```bash
cargo test
```

# Road Race

Assets derived from:

```
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

https://github.com/ghubnerr/getting-rusty/assets/91924667/202ca853-cce5-48e9-bf46-39569c9c6d28
