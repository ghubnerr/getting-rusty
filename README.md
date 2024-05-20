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
│   ├── hasher.rs
│   └── stacks.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
```

### Topics Included

- Binary Trees and Balanced Binary Trees [`bst.rs`](./data-structures-and-algorithms/src/bst.rs)
- Dynamic Programming (simple) [`dp.rs`](./data-structures-and-algorithms/src/dp.rs)
- Huffman Encoding [`encoding.rs`](./data-structures-and-algorithms/src/encoding.rs)
- Graphs and Graph Algorithms [`graphs.rs`](./data-structures-and-algorithms/src/graphs.rs)
- Hashmaps (self-implemented) [`hashmaps.rs`](./data-structures-and-algorithms/src/hashmaps.rs)
- LinkedLists and Doubly Linked Lists [`linkedlists.rs`](./data-structures-and-algorithms/src/linkedlists.rs)
- Pseudo Random Number Generator [`rand_gen.rs`](./data-structures-and-algorithms/src/rand_gen.rs)
- Hashing and Hasher [`hasher.rs`](./data-structures-and-algorithms/src/hasher.rs)
- Sorting algorithms (Bubble, Merge and Quicksort) -- the latter has a thread-pool impl. [`sorting.rs`](./data-structures-and-algorithms/src/sorting.rs)
- Stacks data structure [`stacks.rs`](./data-structures-and-algorithms/src/stacks.rs)

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
