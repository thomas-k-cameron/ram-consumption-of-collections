# Overview
This crate demonstrates that btree/hashmap consumes more than twice as much ram when compared to vec/vecdeque/linkedlist.

Following result was produced on `Linux 3bf76b2c8e56 5.4.72-microsoft-standard-WSL2 #1 SMP Wed Oct 28 23:40:43 UTC 2020 x86_64 GNU/Linux` with `cargo run --release`
| about                         | in bytes  |
| ----------------------------- | --------- |
| without collections           | 1077      |
| hashmap with EmptyStruct      | 18875461  |
| hashmap with BigStruct        | 455083077 |
| btreemap with EmptyStruct     | 19619605  |
| btreemap with BigStruct       | 400946837 |
| vector with EmptyStruct       | 8389685   |
| vector with BigStruct         | 226493493 |
| vector deque with EmptyStruct | 8389685   |
| vector deque with BigStruct   | 226493493 |
| linked list with EmptyStruct  | 24001077  |
| linked list with BigStruct    | 232001077 |

*Note: `without collection` refers to number of bytes allocated without the collections.*

# Methodology
Every collections are inserted with 1_000_000 items whose key is an i64 and value is a default value of EmptyStruct or BigStruct.

## Ram Usage
Ram usage is tracked by incrementing/decrementing a counter when allocation/deallocation happens; It is implemented on top of Jemalloc

## EmptyStruct
Empty struct is a struct without any fields
```rust
struct EmptyStruct
```

## BigStruct
BigStruct is a struct with 24 fields that holds i64.
