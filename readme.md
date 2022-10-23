# Overview
This crate demonstrates that btree/hashmap consumes more than twice as much ram when compared to vec/vecdeque/linkedlist.

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