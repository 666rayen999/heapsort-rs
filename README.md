# HeapSort

This repository contains an implementation of the Heap Sort algorithm for generic types in Rust.
Heap Sort is the fastest i guess with O(n log n) and O(1) space complexity unlike quicksort.

### Features
- **Generic Type Support**: The algorithm works for any type that implements the `PartialOrd` trait.
- **In-Place Sorting**: The sorting is done in place, requiring no additional space.
- **Efficient**: It operates in O(n log n) time complexity.

### Usage
```rust
fn main() {
    let mut array = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    array.heap_sort();
    println!("{:?}", array);
}
```

### Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

### License
This project is licensed under the MIT License.
